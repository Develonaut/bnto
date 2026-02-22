import { BntoGallery } from "./_components/BntoGallery";

/**
 * Home page -- gallery of bnto tools.
 *
 * Server component. Reads from the static bnto registry.
 */
export default function HomePage() {
  return (
    <div className="flex flex-1 flex-col gap-6 p-4 lg:p-6">
      <div>
        <h1 className="text-2xl font-semibold">Tools</h1>
        <p className="mt-1 text-sm text-muted-foreground">
          Pick a tool, drop your files, and get results instantly.
        </p>
      </div>
      <BntoGallery />
    </div>
  );
}
