import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@bnto/ui";

const FAQS = [
  {
    question: "Where are my files processed?",
    answer:
      "On your machine. The CLI runs natively on your system. The browser version uses Rust compiled to WebAssembly. Either way, your files are processed locally and never uploaded anywhere.",
  },
  {
    question: "Is it really free?",
    answer:
      "Yes. Every recipe is free, unlimited, forever. No signup, no watermarks, no quality reduction, no daily caps. The engine runs on your machine, so it costs nothing to operate.",
  },
  {
    question: "What file types are supported?",
    answer:
      "Images: JPEG, PNG, WebP, GIF, AVIF, BMP, and TIFF for compression, resizing, and format conversion. Data: CSV files for cleaning and column renaming. Files: any file type for batch renaming. Video: download via yt-dlp (CLI only). More node types are coming.",
  },
  {
    question: "Can I build my own recipes?",
    answer:
      "Yes. Recipes are .bnto.json files that compose nodes into multi-step pipelines. Chain a resize into a format conversion into a compression, all in one recipe. Share them as files or contribute them to the repo.",
  },
  {
    question: "Is bnto open source?",
    answer:
      "Yes. The entire engine, CLI, and web app are MIT licensed. You can inspect the source code, verify how recipes work, fork it, or contribute.",
  },
  {
    question: "Will bnto always be free?",
    answer:
      "All recipes are free and unlimited. The engine runs on your machine. Managed cloud execution for server-side features may come later, but the core engine and CLI will always be free and open source.",
  },
  {
    question: "How do I install it?",
    answer:
      "Run cargo install bnto to install the CLI. Or use the browser version on this site, no install needed.",
  },
  {
    question: "Can I process multiple files at once?",
    answer:
      "Yes. Pass multiple files to bnto run or drop them in the browser. bnto processes them all in a single batch.",
  },
] as const;

export function FAQAccordion() {
  return (
    <Accordion type="single" collapsible className="w-full">
      {FAQS.map((item, i) => (
        <AccordionItem key={i} value={`faq-${i}`}>
          <AccordionTrigger className="text-left hover:no-underline">
            {item.question}
          </AccordionTrigger>
          <AccordionContent className="text-muted-foreground">{item.answer}</AccordionContent>
        </AccordionItem>
      ))}
    </Accordion>
  );
}
