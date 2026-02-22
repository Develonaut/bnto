import { DashedLine } from "@bnto/ui/dashed-line";
import { BntoGallery } from "./_components/BntoGallery";

/**
 * Home page -- hero section + gallery of bnto tools.
 *
 * Server component. Reads from the static bnto registry.
 */
export default function HomePage() {
  return (
    <div className="flex flex-1 flex-col">
      {/* Hero */}
      <div className="px-6 py-20 text-center lg:px-8">
        <h1 className="font-display text-4xl font-bold tracking-tight text-foreground sm:text-5xl">
          Free Online Tools for Everyday Tasks
        </h1>
        <p className="mx-auto mt-4 max-w-lg text-muted-foreground">
          Compress images, clean CSVs, rename files, and more. No signup, no
          upload limits — just drop your files and go.
        </p>
      </div>

      <DashedLine className="mx-6 lg:mx-8" />

      {/* Gallery */}
      <div className="flex flex-col gap-6 p-6 lg:p-8">
        <BntoGallery />
      </div>
    </div>
  );
}
