import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Next.js with custom directive DEMO",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ja">
      <body>{children}</body>
    </html>
  );
}
