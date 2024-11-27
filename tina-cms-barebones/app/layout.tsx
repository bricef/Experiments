import Link from "next/link";
import React from "react";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <head>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.min.css"/>
      </head>
      <body
        style={{
          margin: "3rem",
        }}
      >
        <header>
          <Link href="/">Home</Link>
          {" | "}
          <Link href="/posts">Posts</Link>
          {" | "}
          <Link href="/about">About</Link>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
