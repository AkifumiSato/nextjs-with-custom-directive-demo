export default function Home() {
  logging("test", 999);

  return (
    <div>
      <h1>Next.js with custom directive DEMO</h1>
    </div>
  );
}

function logging(..._arg: unknown[]) {
  "use debug";
}
