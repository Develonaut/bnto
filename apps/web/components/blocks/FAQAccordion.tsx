import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@bnto/ui";

const FAQS = [
  {
    question: "Where are my files processed?",
    answer:
      "Everything runs in your browser using Rust compiled to WebAssembly — your files are processed on your device and never uploaded anywhere. Future premium recipes may use server-side processing for things browsers can't do, like AI or video.",
  },
  {
    question: "Is it really free?",
    answer:
      "Yes. Every recipe that runs in your browser is free, unlimited, forever. No signup, no watermarks, no quality reduction, no daily caps. We will never put limits on browser-based recipes.",
  },
  {
    question: "What file types are supported?",
    answer:
      "Images: JPEG, PNG, WebP, GIF, AVIF, BMP, and TIFF for compression, resizing, and format conversion. Data: CSV files for cleaning and column renaming. Files: any file type for batch renaming. More node types are coming.",
  },
  {
    question: "Can I build my own recipes?",
    answer:
      "Custom recipe composition is coming soon. Today you can use any of the curated recipes — each one chains nodes into a multi-step pipeline that runs entirely in your browser.",
  },
  {
    question: "Is bnto open source?",
    answer:
      "Yes. The entire engine and web app are MIT licensed. You can inspect the source code, verify how recipes work, or contribute.",
  },
  {
    question: "Will bnto always be free?",
    answer:
      "All browser-based recipes are free and unlimited — no signup, no caps. A Pro tier is planned for features that require server-side processing (AI, video, shell commands), persistence, and team collaboration.",
  },
  {
    question: "Do I need an account?",
    answer:
      "No. Drop your files and run any recipe immediately. Create an account if you want to save favorites or access execution history.",
  },
  {
    question: "Can I process multiple files at once?",
    answer:
      "Yes. Drop as many files as you want. bnto processes them all in a single batch. Results download as a ZIP when there are multiple output files.",
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
