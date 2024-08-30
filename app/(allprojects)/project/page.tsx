'use client';
import { Button } from '@/components/ui/button';
import { ChevronDown, Info, Play, Plus, Search, CircleStop} from 'lucide-react';
import Link from 'next/link';
import { AddDependencydialogbox } from '@/components/addDependencydialogbox';
import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useRef, useState } from 'react';
import { useProjectAnalyzer } from '@/lib/projectDetails';
import { Input } from '@/components/ui/input';
import { UpdateDependencyDialog } from '@/components/updateDependencyDialog';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog"
import { listen } from '@tauri-apps/api/event';

const vsCodeLaunch = async (projectId: string) => {
  const allProjectPath = localStorage.getItem('projectsPath');
  console.log(allProjectPath);
  if (allProjectPath) {
    const projectPath = allProjectPath + '/' + projectId;
    await invoke("launch_vscode", { projectPath });
  }
};

const explorerLaunch = async (projectId: string) => {
  const allProjectPath = localStorage.getItem('projectsPath');
  console.log(allProjectPath);
  if (allProjectPath) {
    const projectPath = `${allProjectPath}/${projectId}/`;
    await invoke("open_file_explorer", { projectPath });
  }
};

export default function Page() {
  const [activeTab, setActiveTab] = useState('overview');
  const { projectName, isRunning, pid, setIsRunning, setPid, projectInfo, error, terminalOutput, setTerminalOutput, appendTerminalOutput, resetTerminalOutput } = useProjectAnalyzer();
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [isUpdateDialogOpen, setIsUpdateDialogOpen] = useState(false); // State for update dialog
  const [selectedDependency, setSelectedDependency] = useState<string | null>(null); // Manage selected dependency
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [dependencyToDelete, setDependencyToDelete] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState(''); // State for search query
  const terminalRef = useRef<HTMLPreElement>(null);

  useEffect(() => {
    if (terminalRef.current) {
      terminalRef.current.scrollTop = terminalRef.current.scrollHeight;
    }
  }, [terminalOutput]);

  const localHostLaunch = async () => {
    const allProjectPath = localStorage.getItem('projectsPath');
    if (allProjectPath) {
      const projectPath = `${allProjectPath}/${projectName}`;
      try {
        setActiveTab('terminal');
        resetTerminalOutput(); // Reset terminal output before starting
        const result: number = await invoke('start_project', { projectPath });
        setPid(result);
        setIsRunning(true);

        // Store PID and running status in local storage
        const projectData = {
          pid: result,
          isRunner: true
        };

        console.log(`PID in launch ${projectName} is`, result);
        console.log("projectData", projectData);
        localStorage.setItem(projectName as string, JSON.stringify(projectData));

      } catch (error) {
        console.error('Error running command:', error);
        appendTerminalOutput(`Error running command: ${error}`);
      }
    }
  };

  const handleAddDependency = () => {
    setIsDialogOpen(true); // Open the dialog
  };

  const closeDialog = () => {
    setIsDialogOpen(false); // Close the dialog
  };

  const handleDependencySubmit = async (name: string, version: string) => {
    console.log('Adding dependency:', name, version);
    const allProjectPath = localStorage.getItem('projectsPath');
    if (allProjectPath && projectInfo) { // Add null check for projectInfo
      const projectPath = `${allProjectPath}/${projectName}`;
      try {
        setIsDialogOpen(false); // Close the dialog
        setActiveTab('terminal');
        resetTerminalOutput(); // Reset terminal output before starting
        await invoke('install_dependency', { projectPath, runtime: projectInfo.runtime, dependency: name, version });
      } catch (error) {
        console.error('Error installing dependency:', error);
        appendTerminalOutput(`Error installing dependency: ${error}`);
      }
    }
  };

  // Listen for install status updates
  useEffect(() => {
    const unlisten = listen<string>('install_status', (event) => {
      appendTerminalOutput(event.payload);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const openUpdateDialog = (dependency: string) => {
    setSelectedDependency(dependency);
    setIsUpdateDialogOpen(true); // Open the dialog for updating
  };

  const handleUpdateDependency = async (name: string, version: string) => {
    console.log(`Updating dependency: ${name} to version ${version}`);
    const allProjectPath = localStorage.getItem('projectsPath');
    if (allProjectPath && projectInfo) { // Add null check for projectInfo
      const projectPath = `${allProjectPath}/${projectName}`;
      try {
        setIsUpdateDialogOpen(false); // Close the dialog
        setActiveTab('terminal');
        resetTerminalOutput(); // Reset terminal output before starting
        await invoke('update_dependency', { projectPath, runtime: projectInfo.runtime, dependency: name, version });
      } catch (error) {
        console.error('Error updating dependency:', error);
        appendTerminalOutput(`Error updating dependency: ${error}`);
      }
    }
  };

  // Listen for update status updates
  useEffect(() => {
    const unlisten = listen<string>('update_status', (event) => {
      appendTerminalOutput(event.payload);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const closeUpdateDialog = () => {
    setIsUpdateDialogOpen(false);
  };

  const handleDeleteDependency = (dependency: string) => {
    setDependencyToDelete(dependency);  // Set the dependency to delete
    setIsDeleteDialogOpen(true);  // Open the delete confirmation dialog
  };

  const confirmDeleteDependency = async () => {
    if (dependencyToDelete) {
      console.log(`Deleting dependency: ${dependencyToDelete}`);
      const allProjectPath = localStorage.getItem('projectsPath');
      if (allProjectPath && projectInfo) { // Add null check for projectInfo
        const projectPath = `${allProjectPath}/${projectName}`;
        try {
          setIsDeleteDialogOpen(false); // Close the dialog
          setActiveTab('terminal');
          resetTerminalOutput(); // Reset terminal output before starting
          await invoke('delete_dependency', { projectPath, runtime: projectInfo.runtime, dependency: dependencyToDelete });
        } catch (error) {
          console.error('Error deleting dependency:', error);
          appendTerminalOutput(`Error deleting dependency: ${error}`);
        }
      }
    }
  };

  // Listen for delete status updates
  useEffect(() => {
    const unlisten = listen<string>('delete_status', (event) => {
      appendTerminalOutput(event.payload);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  const closeDeleteDialog = () => {
    setIsDeleteDialogOpen(false);
    setDependencyToDelete(null);
  };

  
  const stopLocalHost = async () => {
    try {
      console.log("Stopping project with PID:", pid);
      if (!pid) return;
      await invoke('close_project', { pid: pid });
      setIsRunning(false);
      setPid(null);
      resetTerminalOutput(); // Reset terminal output after stopping
  
      // Update local storage to reflect stopped status
      const projectData = {
        pid: null,
        isRunner: false
      };
      localStorage.removeItem(`${projectName}_terminalOutput`);
      localStorage.removeItem(projectName as string);
    } catch (error) {
      console.error('Error stopping command:', error);
      appendTerminalOutput(`Error stopping command: ${error}`);
    }
  };
  
  const filteredPackages = projectInfo?.packages.filter(pkg =>
    pkg.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  return (
    <div className="bg-gray-900 text-gray-200 min-h-screen">
      {/* Header Section */}
      <header className="flex flex-col md:flex-row justify-between items-center mb-4 p-6">
        <h1 className="text-3xl font-bold">{projectName}</h1>
        <Button
          onClick={isRunning ? stopLocalHost : localHostLaunch}
          className={`text-white px-4 py-2 rounded-md flex items-center transition-colors mt-4 md:mt-0 ml-auto ${
            isRunning ? 'bg-red-600 hover:bg-red-700' : 'bg-purple-600 hover:bg-purple-700'
          }`}
        >
          {isRunning ? <CircleStop className="w-5 h-5 mr-2" /> : <Play className="w-5 h-5 mr-2" />}
          {isRunning ? 'Stop Site' : 'Start Site'}
        </Button>
      </header>

      {/* Navigation Section */}
      <nav className="mb-4 p-6">
        <ul className="flex flex-col md:flex-row space-y-4 md:space-y-0 md:space-x-6">
          <li>
            <Link
              href="#"
              onClick={() => explorerLaunch(projectName as string)}
              className="text-purple-400 hover:underline"
            >
              Go to Site Folder
            </Link>
          </li>
          <li>
            <Link
              href="#"
              onClick={() => vsCodeLaunch(projectName as string)}
              className="text-purple-400 hover:underline"
            >
              VS Code
            </Link>
          </li>
        </ul>
      </nav>

      {/* Tabs Section */}
      <div className="p-6">
        <ul className="m-0 p-0 flex flex-col md:flex-row space-y-4 md:space-y-0 md:space-x-6 border-b border-gray-700">
          <li className={`pb-2 ${activeTab === 'overview' ? 'border-b-2 border-purple-500' : ''}`}>
            <Link href="#" onClick={() => setActiveTab('overview')} className="text-gray-400 hover:text-purple-300">
              Overview
            </Link>
          </li>
          <li className={`pb-2 ${activeTab === 'packages' ? 'border-b-2 border-purple-500' : ''}`}>
            <Link href="#" onClick={() => setActiveTab('packages')} className="text-gray-400 hover:text-purple-300">
              Packages
            </Link>
          </li>
          <li className={`pb-2 ${activeTab === 'terminal' ? 'border-b-2 border-purple-500' : ''}`}>
            <Link href="#" onClick={() => setActiveTab('terminal')} className="text-gray-400 hover:text-purple-300">
              Terminal
            </Link>
          </li>
        </ul>
      </div>

      {/* Content Section */}
      <div className="p-6">
        {activeTab === 'overview' && (
          <div>
            {error && <p className="text-red-500 mb-4">{error}</p>}
            {projectInfo && (
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="space-y-6">
                  <DetailRow label="Framework" value={projectInfo.framework} />
                  <DetailRow label="Runtime" value={projectInfo.runtime}  />
                </div>
              </div>
            )}
          </div>
        )}

        {activeTab === 'packages' && (
          <div>
            <div className="flex flex-col md:flex-row justify-between items-center mb-4">
              <Button
                onClick={handleAddDependency}
                className="bg-purple-600 text-white px-4 py-2 rounded-md flex items-center hover:bg-purple-700 transition-colors"
              >
                <Plus className="w-5 h-5 mr-2" />
                Add Dependency
              </Button>
              <div className="flex items-center relative mt-4 md:mt-0">
                <Search className="absolute left-3 w-5 h-5 text-gray-500" />
                <Input
                  type="search"
                  placeholder="Search packages"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="bg-gray-800 w-full py-2 pl-10 text-sm text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-gray-600"
                />
              </div>
            </div>
            <div className="h-96 overflow-y-auto">
              {filteredPackages && (
                <div className="grid grid-cols-1 md:grid-cols-1 gap-6">
                  <div className="space-y-6">
                    {filteredPackages.map((pkg, index) => (
                      <DetailRow
                        key={index}
                        label={pkg.name}
                        value={pkg.version}
                        updateButton={true} // Show the update button
                        onUpdate={() => openUpdateDialog(pkg.name)}  // Pass the update function
                        onDelete={() => handleDeleteDependency(pkg.name)}
                      />
                    ))}
                  </div>
                </div>
              )}
            </div>
          </div>
        )}
        <UpdateDependencyDialog
          isOpen={isUpdateDialogOpen}
          onClose={closeUpdateDialog}
          onSubmit={handleUpdateDependency}
          dependency={selectedDependency || ''} // Pass selected dependency or empty string
        />
        <AddDependencydialogbox isOpen={isDialogOpen} onClose={closeDialog} onSubmit={handleDependencySubmit} />

        <AlertDialog open={isDeleteDialogOpen} onOpenChange={setIsDeleteDialogOpen}>
          <AlertDialogContent className='bg-gray-800'>
            <AlertDialogHeader>
              <AlertDialogTitle className='text-red-500'>Are you absolutely sure?</AlertDialogTitle>
              <AlertDialogDescription className='text-gray-300'>
                This action cannot be undone. This will permanently delete the selected dependency <b>{dependencyToDelete}</b>.
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel onClick={closeDeleteDialog} className='bg-purple-600 hover:bg-purple-700'>Cancel</AlertDialogCancel>
              <AlertDialogAction onClick={confirmDeleteDependency} className='bg-red-500 hover:bg-red-600'>Delete</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
        {activeTab === 'terminal' && (
          <div>
            <pre
              ref={terminalRef}
              className="bg-black text-white p-4 rounded-lg overflow-y-auto max-h-80 h-40 sm:h-60 md:h-80 lg:h-96"
            >
              <code>{terminalOutput || 'Terminal output will appear here...'}</code>
            </pre>
          </div>
        )}
      </div>
    </div>
  );
}

// Helper component for detail rows
const DetailRow = ({
  label,
  value,
  linkText,
  infoIcon,
  dropdownIcon,
  toggle,
  dotIcon,
  updateButton,
  onUpdate,
  onDelete
}: {
  label: string;
  value: string;
  linkText?: string;
  infoIcon?: boolean;
  dropdownIcon?: boolean;
  toggle?: boolean;
  dotIcon?: boolean;
  updateButton?: boolean;
  onUpdate?: () => void;
  onDelete?: () => void;
}) => (
  <div className="flex flex-col md:flex-row space-y-4 md:space-y-0 justify-between items-center bg-gray-800 p-4">
    <span className="text-gray-400">{label}</span>
    <div className="flex items-center space-x-3">
      <span className="text-gray-300 flex flex-1 items-center">{value}</span>
      {linkText && (
        <Link href="#" className="text-purple-400 hover:underline">
          {linkText}
        </Link>
      )}
      {infoIcon && <Info className="w-4 h-4 text-gray-400" />}
      {dropdownIcon && <ChevronDown className="w-4 h-4 text-gray-400" />}
      {toggle && (
        <div className="w-10 h-6 bg-gray-700 rounded-full flex items-center">
          <div className="w-4 h-4 rounded-full bg-gray-400 ml-1"></div>
        </div>
      )}
      {dotIcon && <div className="w-2 h-2 bg-gray-400 rounded-full"></div>}

      {updateButton && onUpdate && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" className='bg-purple-600 hover:bg-purple-700'>Options</Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="w-10 bg-gray-800">
            <DropdownMenuItem onClick={onUpdate} className='text-purple-400 '>
              Update
            </DropdownMenuItem>
            <DropdownMenuItem onClick={onDelete} className='text-red-500'>
              Delete
            </DropdownMenuItem>
            {/* Add more dropdown items as needed */}
          </DropdownMenuContent>
        </DropdownMenu>
      )}
    </div>
  </div>
);

