import { Inter } from "next/font/google";
import "../../globals.css";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Terminal } from "lucide-react"; // Assuming you are using react-icons

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-gray-900 overflow-hidden`}>
        <div className="flex flex-col  min-h-screen p-2">
          <Alert variant="destructive">
            <Terminal className="h-4 w-4" />
            <AlertTitle>Warning!</AlertTitle>
            <AlertDescription>
              Please delete any <b>existing NVM and Node.js</b> installations before proceeding.
            </AlertDescription>
          </Alert>
          <main className="">
           {children}
          </main>
        </div>
      </body>
    </html>
  );
}
