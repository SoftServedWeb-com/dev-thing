import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "../globals.css";
import Sidebar from "@/components/sidebar";
import { ProjectsProvider } from "@/lib/useProject";
import ErrorBoundary from "@/lib/errorBound";


const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "dev-thing",
  description: "dev-thing is an open source LocalWP but for Node JS Applications. Effortlessly manage your Node JS applications with a simple and intuitive interface.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-gray-900`}>
      <ErrorBoundary>
        <ProjectsProvider>
          <div className="flex h-screen">
            <Sidebar />
            <main className="flex-1  overflow-hidden">{children}</main>
          </div>
        </ProjectsProvider>
      </ErrorBoundary>
      </body>
    </html>
  );
}