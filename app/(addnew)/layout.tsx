import { Inter } from "next/font/google";
import "../globals.css";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${inter.className} bg-gray-900`}>
        <div className="flex items-center justify-center h-screen">
          <main className="p-9">{children}</main>
        </div>
      </body>
    </html>
  );
}
