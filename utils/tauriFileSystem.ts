import { readDir } from '@tauri-apps/api/fs';

export async function getProjects(): Promise<string[]> {
  const projectsPath = "/home/ron-tennyson/Local-Projects";
  if (!projectsPath) {
    throw new Error('Projects path not set');
  }

  try {
    console.log("Bhenchod")
    const entries = await readDir(projectsPath);
    return entries
      .filter(entry => entry.children !== undefined)  // Only include directories
      .map(entry => entry.name!)
      .filter((name): name is string => name !== undefined);
  } catch (err) {
    console.error('Error reading projects directory:', err);
    return [];

  }
}