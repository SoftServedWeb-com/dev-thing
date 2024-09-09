import { invoke } from '@tauri-apps/api/tauri';


export const ideLaunch = async (projectId: string, platform: string,ide: string) => {
    const allProjectPath = localStorage.getItem('projectsPath');
    console.log(allProjectPath);
    if (allProjectPath) {
      let projectPath;
      
      if (platform === 'win32') {
         projectPath = allProjectPath + '\\' + projectId;
      } else {
         projectPath = allProjectPath + '/' + projectId;
      }
      console.log("projectPath in ideLaunch", projectPath,ide)
      await invoke("launch_ide", { projectPath,ide });
    }
  };
  
export const explorerLaunch = async (projectId: string, platform: string) => {
    const allProjectPath = localStorage.getItem('projectsPath');
    console.log(allProjectPath);
    if (allProjectPath) {
      let projectPath;
      
      if (platform === 'win32') {
        projectPath = allProjectPath + '\\' + projectId;
      } else {
        projectPath = allProjectPath + '/' + projectId;
      }
      await invoke("open_file_explorer", { projectPath });
    }
  };
  
  export function makeUrlsClickable(text: any) {
    const urlRegex = /(https?:\/\/[^\s<]+)/g; // Updated regex to stop at whitespace or '<'
    return text.replace(urlRegex, (url: any) => {
      if (typeof window !== 'undefined' && window.__TAURI__ && (window.__TAURI__ as any).shell) {
        return `<a href="#" onclick="window.__TAURI__.shell.open('${url}'); return false;" style="color: #60a5fa; text-decoration: underline;">${url}</a>`;
      } else {
        return `<a href="${url}" target="_blank" style="color: #60a5fa; text-decoration: underline;">${url}</a>`;
      }
    });
  }
  