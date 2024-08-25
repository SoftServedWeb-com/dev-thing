'use client';

import { Button } from '@/components/ui/button';
import { ChevronDown, Info, Play } from 'lucide-react';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';

import { invoke } from '@tauri-apps/api/tauri';
const vsCodeLaunch = async (projectId: string) => {
        const allProjectPath = localStorage.getItem('projectsPath');
        console.log(allProjectPath)
        if (allProjectPath) {
          const projectPath = allProjectPath + '/' + projectId;
          await invoke("launch_vscode",{projectPath})
        }
}

const explorerLaunch = async (projectId: string) => {
  const allProjectPath = localStorage.getItem('projectsPath');
  console.log(allProjectPath)
  if (allProjectPath) {
    const projectPath = `${allProjectPath}/${projectId}/`;
    await invoke("open_file_explorer",{projectPath})
  }
}

const localHostLaunch = async (projectId: string) => {
  const allProjectPath = localStorage.getItem('projectsPath');
  if (allProjectPath) {
    const projectPath = `${allProjectPath}/${projectId}`;
    try {
      const runtime = await invoke('detect_runtime', { projectPath });
      let command = '';

      switch (runtime) {
        case 'pnpm':
          command = 'pnpm run dev';
          break;
        case 'bun':
          command = 'bun run dev';
          break;
        case 'npm':
          command = 'npm run dev';
          break;
        default:
          console.error('Unsupported runtime detected');
          return;
      }

      await invoke('run_command', { projectPath, command });
    } catch (error) {
      console.error('Error detecting runtime or running command:', error);
    }
  }
};


export default function Page() {
  const searchParams = useSearchParams();
  const projectId = searchParams.get('page'); // Use the parameter name

  return (
    <div className="bg-gray-900 text-gray-200 min-h-screen p-6">
      {/* Header Section */}
      <header className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">{projectId}</h1>
        <Button onClick={() => localHostLaunch(projectId as string)} className="bg-purple-600 text-white px-4 py-2 rounded-md flex items-center hover:bg-purple-700 transition-colors">
          <Play className="w-5 h-5 mr-2" />
          Start Site
        </Button>
      </header>

      {/* Navigation Section */}
      <nav className="mb-8">
        <ul className="flex space-x-6">
          <li><Link href="#" onClick={() => explorerLaunch(projectId as string)} className="text-purple-400 hover:underline">Go to Site Folder</Link></li>
          <li><Link href="#" onClick={() => vsCodeLaunch(projectId as string)}  className="text-purple-400 hover:underline">VS Code</Link></li>
        </ul>
      </nav>

      {/* Tabs Section */}
      <div className="mb-8">
        <ul className="flex space-x-6 border-b border-gray-700">
          <li className="pb-2 border-b-2 border-purple-500">
            <Link href="#" className="text-purple-400 hover:text-purple-300">Overview</Link>
          </li>
          {/* <li>
            <Link href="#" className="text-gray-400 hover:text-gray-300">Database</Link>
          </li> */}
          <li>
            <Link href="#" className="text-gray-400 hover:text-gray-300">Packages</Link>
          </li>
        </ul>
      </div>

      {/* Details Section */}
      {/* <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="space-y-6">
          <DetailRow label="Site Domain" value="yoyo.local" linkText="Change" />
          <DetailRow label="SSL" value="yoyo.local.crt" linkText="Trust" infoIcon />
          <DetailRow label="Web Server" value="nginx" dropdownIcon />
          <DetailRow label="PHP Version" value="8.1.29" dropdownIcon />
          <DetailRow label="Database" value="MySQL 8.0.16" />
        </div>
        <div className="space-y-6">
          <DetailRow
            label="One-Click Admin"
            value="Off"
            toggle
            dropdownIcon
            infoIcon
          />
          <DetailRow label="WordPress Version" value="N/A" dotIcon />
          <DetailRow label="Multisite" value="No" />
          <DetailRow
            label="Xdebug"
            value="Off"
            toggle
            infoIcon
          />
        </div>
      </div> */}

      {/* Footer Section */}
      {/* <div className="mt-8 flex justify-between items-center">
        <div className="flex items-center space-x-4">
          <div className="w-10 h-6 bg-gray-700 rounded-full flex items-center justify-start">
            <div className="w-4 h-4 rounded-full bg-gray-400 ml-1"></div>
          </div>
          <span className="text-gray-400">Live Link</span>
          <Link href="#" className="text-purple-400 hover:underline">Enable</Link>
        </div>
        <div className="flex space-x-4">
          <Button className="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700 transition-colors">Pull</Button>
          <Button className="bg-purple-600 text-white px-4 py-2 rounded-md hover:bg-purple-700 transition-colors">Push</Button>
        </div>
      </div> */}
    </div>
  );
}

// Helper component for detail rows
const DetailRow = ({ label, value, linkText, infoIcon, dropdownIcon, toggle, dotIcon }: {
  label: string;
  value: string;
  linkText?: string;
  infoIcon?: boolean;
  dropdownIcon?: boolean;
  toggle?: boolean;
  dotIcon?: boolean;
}) => (
  <div className="flex justify-between items-center">
    <span className="text-gray-400">{label}</span>
    <div className="flex items-center space-x-2">
      <span className="text-gray-300">{value}</span>
      {linkText && <Link href="#" className="text-purple-400 hover:underline">{linkText}</Link>}
      {infoIcon && <Info className="w-4 h-4 text-gray-400" />}
      {dropdownIcon && <ChevronDown className="w-4 h-4 text-gray-400" />}
      {toggle && (
        <div className="w-10 h-6 bg-gray-700 rounded-full flex items-center">
          <div className="w-4 h-4 rounded-full bg-gray-400 ml-1"></div>
        </div>
      )}
      {dotIcon && <div className="w-2 h-2 bg-gray-400 rounded-full"></div>}
    </div>
  </div>
);
