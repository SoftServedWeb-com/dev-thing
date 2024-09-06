"use client";
import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { useRouter } from "next/navigation";
import { invoke } from '@tauri-apps/api/tauri';
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";

const SetupPage = () => {
  const [nvmInstalled, setNvmInstalled] = useState<boolean>(false);
  const [nodeVersion, setNodeVersion] = useState<string>('');
  const [runtime, setRuntime] = useState<string>('');
  const [isNodeDetected, setIsNodeDetected] = useState<boolean>(false);
  const [shouldManageNode, setShouldManageNode] = useState<boolean | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true); // New loading state
  const router = useRouter();

  useEffect(() => {
    const checkNodeInstallation = async () => {
      try {
        const nodeInstalled = await invoke('check_node_installed'); // Tauri function to check Node installation
        const nvmInstalled = await invoke('check_nvm_installed'); // Tauri function to check NVM installation
        console.log(nodeInstalled, nvmInstalled);
        if (nodeInstalled) {
          setIsNodeDetected(false);
        }
        if (nvmInstalled === true) {
          setNvmInstalled(true);
          localStorage.setItem('nvm', 'true');
        }
      } catch (error) {
        console.error("Failed to check Node installation:", error);
      } finally {
        setIsLoading(false); // Set loading to false after fetch is complete
      }
    };

    checkNodeInstallation();
  }, [router]);

  //TODO: Install Node.js and NVM
  const handleManageNode = async () => {

    if(!nvmInstalled) {
      try {
        await invoke('install_nvm');
      } catch (error) {
        console.error("Failed to install NVM:", error);
      }
      const nvmInstalled = await invoke('check_nvm_installed');
      if(nvmInstalled) {
        setNvmInstalled(true);
        localStorage.setItem('nvm', 'true');
      }
    }
    else {
      try {
        await invoke('install_node_with_nvm');
      } catch (error) {
        console.error("Failed to install Node with NVM:", error);
      }
      const nodeInstalled = await invoke('check_node_installed');
      if(nodeInstalled) {
        setIsNodeDetected(false);
        localStorage.setItem('nodeInstalled', 'true');
      }
    }
    if(isNodeDetected && nvmInstalled) {
      router.replace('/');
    }
  };

  const handleContinueWithExistingNode = () => {
    localStorage.setItem('nvm', 'true');
    localStorage.setItem('nodeInstalled', 'true');
    router.replace('/');
  };

  const handleInstallNvm = async () => {
    try {
      // Simulate NVM installation
      console.log("NVM is being installed");
      // Assume installation is successful
      localStorage.setItem('nvm', 'true');
      setNvmInstalled(true);
    } catch (error) {
      console.error("Failed to install NVM:", error);
    }
  };

  const handleInstallNode = async () => {
    try {
      // Simulate Node installation
      console.log(`Installing Node version ${nodeVersion} with runtime ${runtime}`);
      // Assume installation is successful
      localStorage.setItem('nodeInstalled', 'true');
      router.push('/');
    } catch (error) {
      console.error("Failed to install Node:", error);
    }
  };

  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-900 text-purple-200">
        <h1 className="text-3xl font-bold mb-6">Loading...</h1>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center mt-28 h-screen bg-gray-900 text-purple-200">
      {!nvmInstalled ? (
        <>
          <h1 className="text-3xl font-bold mb-6">Install NVM</h1>
          <Button
            onClick={handleInstallNvm}
            className="bg-gradient-to-r from-purple-500 to-purple-700 text-white px-6 py-3 rounded-lg shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
          >
            Install NVM
          </Button>
        </>
      ) : (
        <>
          <h1 className="text-3xl font-bold mb-6">Setup Node Environment</h1>
          <div className="flex flex-col gap-4 mb-6">
            <select
              value={nodeVersion}
              onChange={(e) => setNodeVersion(e.target.value)}
              className="bg-gray-800 text-black border-purple-500 focus:border-purple-400 p-2 rounded"
            >
              <option value="" disabled>Select Node Version</option>
              <option value="14">14</option>
              <option value="16">16</option>
              <option value="18">18</option>
            </select>
            <select
              value={runtime}
              onChange={(e) => setRuntime(e.target.value)}
              className="bg-gray-800 text-black border-purple-500 focus:border-purple-400 p-2 rounded"
            >
              <option value="" disabled>Select Runtime</option>
              <option value="node">Node</option>
              <option value="deno">Deno</option>
            </select>
          </div>
          <Button
            onClick={handleInstallNode}
            className="bg-gradient-to-r from-purple-500 to-purple-700 text-white px-6 py-3 rounded-lg shadow-lg hover:from-purple-600 hover:to-purple-800 transform hover:scale-105 transition-all"
            disabled={!nodeVersion || !runtime}
          >
            Install
          </Button>
        </>
      )}

      <AlertDialog open={isNodeDetected} onOpenChange={setIsNodeDetected}>
        <AlertDialogContent className='bg-gray-800'>
          <AlertDialogHeader>
            <AlertDialogTitle className='text-red-500'>Node.js Detected</AlertDialogTitle>
            <AlertDialogDescription className='text-gray-300'>
              We have detected Node.js installed. Do you want us to manage Node versions for you? This will require uninstalling the existing Node.js and related packages.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel onClick={handleContinueWithExistingNode} className='bg-purple-600 hover:bg-purple-700'>Continue with Existing Node</AlertDialogCancel>
            <AlertDialogAction onClick={handleManageNode} className='bg-red-500 hover:bg-red-600'>Let Us Manage Node</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
};

export default SetupPage;