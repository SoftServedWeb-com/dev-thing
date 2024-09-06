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

export function UpdateDependencyDialog({ isOpen, onClose, onSubmit, dependency }: {   isOpen: boolean; onClose: () => void; onSubmit: (name: string, version: string) => void; dependency: string}) {
  const [dependencyVersion, setDependencyVersion] = useState('');

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();
    onSubmit(dependency,dependencyVersion); // Call the onSubmit prop
    setDependencyVersion(''); // Clear form fields
    onClose(); // Close the dialog
  };
  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="sm:max-w-[525px]">
        <DialogHeader>
          <DialogTitle>Update Dependency: {dependency}</DialogTitle>
          <DialogDescription>
            Enter the details for the new dependency.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid grid-cols-4 items-center gap-4">
            <Label htmlFor="dependencyVersion" className="text-right text-white">
              Version
            </Label>
            <Input id="dependencyVersion" onChange={(e) => setDependencyVersion(e.target.value)} className="col-span-3" />
          </div>
        </div>
        <DialogFooter>
          <Button type="button" className="bg-purple-600 hover:bg-purple-700" onClick={onClose}>Cancel</Button>
          <Button type="submit" className="bg-purple-600 hover:bg-purple-700" onClick={handleSubmit}>Update Dependency</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
