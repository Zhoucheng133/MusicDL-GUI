import { defineStore } from "pinia";
import { ref } from "vue";
import { message } from '@tauri-apps/plugin-dialog';
import { Command } from '@tauri-apps/plugin-shell';
import { invoke } from '@tauri-apps/api/core';
import { downloadDir, extname, homeDir, join } from "@tauri-apps/api/path";
import { listen } from "@tauri-apps/api/event";
import { remove } from "@tauri-apps/plugin-fs";

export class ListItem{
  constructor(
    public name: string, 
    public artist: string,
    public album: string,
    public url: string,
    public cover: string,
    public duration: string,
  ){}
}

export default defineStore("index", ()=>{
  const client=ref('NeteaseMusicClient');
  const keyword=ref('');
  const loading=ref(false);

  const downloading = ref(false);
  const downloadProgress = ref(0);
  const downloadPath=ref('');
  const list=ref<ListItem[]>([]);
  const encode=ref("mp3 (320k)");
  const saveConfig=ref(false);
  const showProgressDialog=ref(false);

  const search=async ()=>{
    if(keyword.value.length === 0){
      await message('搜索关键词不能为空', { title: '无法搜索', kind: 'error' });
      return;
    }
    loading.value=true;
    const home = await homeDir();
    const args=[
      "-keyword", keyword.value,
      "-client", client.value,
      "-workdir", home,
    ]

    const command = Command.sidecar('binaries/core', args, {
      encoding: "utf-8",
      env: {
        "PYTHONUTF8": "1",
        "LANG": "en_US.UTF-8",
        "LC_ALL": "en_US.UTF-8"
      }
    });
    const output = await command.execute();
    const match = output.stdout.match(/===JSON_START===\r?\n([\s\S]*?)\r?\n===JSON_END===/);
    if(match){
      const jsonData = JSON.parse(match[1]);
      console.log(jsonData);
      list.value=jsonData.map((item: any)=>{
        const formattedTime = item.duration.replace(/^00:/, '');
        return new ListItem(
          item.name,
          item.artist,
          item.album,
          item.url,
          item.cover,
          formattedTime,
        );
      });
    }
    loading.value=false;
  };

  const pathCheck=async (path: string): Promise<boolean> =>{
    return await invoke('path_check', { path });
  }

  const download=async (item: ListItem)=>{

    if(!await pathCheck(downloadPath.value)){
      await message('下载目录不存在或没有权限', { title: '无法下载', kind: 'error' });
      return;
    }

    downloadProgress.value=0;
    downloading.value=true;
    showProgressDialog.value=true;
    
    const cleanUrl = item.url.split('?')[0];
    const ext=await extname(cleanUrl);
    // let path=await join(downloadPath.value, `${item.artist} - ${item.name}.${ext}`);
    let path=await join(downloadPath.value, `temp.${ext}`);

    listen("download-progress", (event) => {
      const progress = event.payload as number;
      downloadProgress.value=progress;
    });

    await invoke('download_file', { url: item.url, savePath: path });
    convert(item, path);
  }

  const cancelDownload=async ()=>{
    showProgressDialog.value=false;
    await invoke("cancel_download");
    downloading.value=false;
    downloadProgress.value=0;
  }

  const convert=async (item: ListItem, filePath: string)=>{
    downloading.value=false;
    const args=[
      "-y",
      "-i", filePath,
      "-i", item.cover,
      '-map', '0:a',
      '-map', '1:0',
      '-metadata', `title=${item.name}`,
      '-metadata', `artist=${item.artist}`,
      '-metadata', `album=${item.album}`,
      '-id3v2_version', '3',
      '-metadata:s:v', 'title=Album cover',
      '-metadata:s:v', 'comment=Cover (front)',
    ];

    switch (encode.value) {
      case "mp3 (320k)":
        args.push(
          '-c:a', 'libmp3lame',
          '-b:a', '320k',
        );
        break;
      case "mp3 (192k)":
        args.push(
          '-c:a', 'libmp3lame',
          '-b:a', '192k',
        );
        break;
      case "mp3 (128k)":
        args.push(
          '-c:a', 'libmp3lame',
          '-b:a', '128k',
        );
        break;
      case "FLAC (原始)":
        args.push(
          '-c:a', 'copy',
        );
    }
    let path=await join(downloadPath.value, `${item.artist} - ${item.name}.${encode.value.includes("FLAC")?"flac": "mp3"}`);
    args.push(
      path
    )

    const command = Command.sidecar('binaries/ffmpeg', args);
    await command.execute();

    await remove(filePath);

    showProgressDialog.value=false;
  }

  const init=async ()=>{
    downloadPath.value=await downloadDir();
  }

  return {
    client,
    keyword,
    search,
    list,
    loading,
    download,
    downloadPath,
    encode,
    saveConfig,
    pathCheck,
    downloading,
    downloadProgress,
    init,
    showProgressDialog,
    cancelDownload
  };
});