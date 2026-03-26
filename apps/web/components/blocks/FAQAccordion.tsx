import { Accordion, AccordionItem, AccordionTrigger, AccordionContent } from "@bnto/ui";

const FAQS = [
  {
    question: "Where are my files processed?",
    answer:
      "Most tools run entirely in your browser using Rust compiled to WebAssembly — your files are processed on your device. Some future tools may use server-side processing for more advanced features.",
  },
  {
    question: "Is it really free?",
    answer:
      "Yes. Every tool that runs in your browser is free, unlimited, forever. No signup, no watermarks, no quality reduction, no daily caps. We will never put limits on browser-based tools.",
  },
  {
    question: "What file types are supported?",
    answer:
      "Images: JPEG, PNG, and WebP for compression, resizing, and format conversion. Data: CSV files for cleaning and column renaming. Files: any file type for batch renaming.",
  },
  {
    question: "Is bnto open source?",
    answer:
      "Yes. The engine is MIT licensed. You can inspect the source code, verify how tools work, or contribute.",
  },
  {
    question: "Will bnto always be free?",
    answer:
      "All current browser tools are free and unlimited — no signup, no caps. We plan to add a paid tier for advanced features that require server-side processing.",
  },
  {
    question: "Do I need an account?",
    answer:
      "No. Drop your files and use any tool immediately. No account, no signup, no email required.",
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
