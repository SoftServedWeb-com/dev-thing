import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState } from "react";

export function AddDependencydialogbox({ isOpen, onClose, onSubmit }: {   isOpen: boolean; onClose: () => void; onSubmit: (name: string, version: string) => void;}) {
  const [dependencyName, setDependencyName] = useState('');
  const [dependencyVersion, setDependencyVersion] = useState('');

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    onSubmit(dependencyName, dependencyVersion); // Call the onSubmit prop
    setDependencyName(''); // Clear form fields
    setDependencyVersion(''); // Clear form fields
    onClose(); // Close the dialog
  };
  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[525px]">
        <DialogHeader>
          <DialogTitle>Add Dependency</DialogTitle>
          <DialogDescription>
            Enter the details for the new dependency.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="dependencyName" className="text-right text-white">
              Dependency Name
            </Label>
            <Input id="dependencyName" className="col-span-3" onChange={(e) => setDependencyName(e.target.value)} required/>
          </div>
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="dependencyVersion" className="text-right text-white">
              Version
            </Label>
            <Input id="dependencyVersion" onChange={(e) => setDependencyVersion(e.target.value)} className="col-span-3" />
          </div>
        </div>
        <DialogFooter>
          <Button type="button" className="bg-purple-600 hover:bg-purple-700" onClick={onClose}>Cancel</Button>
          <Button type="submit" className="bg-purple-600 hover:bg-purple-700" onClick={handleSubmit}>Add Dependency</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
