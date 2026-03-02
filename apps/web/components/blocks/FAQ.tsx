"use client";

import { Accordion } from "@/components/ui/Accordion";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { GITHUB_URL } from "@/lib/copy";

const faqs = [
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

export function FAQ({ headerTag = "h2" }: { headerTag?: "h1" | "h2" }) {
  return (
    <div className="grid gap-12 lg:grid-cols-2 lg:gap-16">
      <Stack gap="md">
        <Heading level={headerTag === "h1" ? 1 : 2}>
          Frequently asked questions
        </Heading>
        <Text color="muted" leading="snug" className="max-w-md">
          Can&apos;t find what you need?{" "}
          <a
            href={GITHUB_URL}
            target="_blank"
            rel="noopener noreferrer"
            className="underline underline-offset-4 transition-colors hover:text-foreground"
          >
            Open an issue on GitHub
          </a>
          .
        </Text>
      </Stack>

      <Accordion type="single" collapsible className="w-full">
        {faqs.map((item, i) => (
          <Accordion.Item key={i} value={`faq-${i}`}>
            <Accordion.Trigger className="text-left hover:no-underline">
              {item.question}
            </Accordion.Trigger>
            <Accordion.Content className="text-muted-foreground">
              {item.answer}
            </Accordion.Content>
          </Accordion.Item>
        ))}
      </Accordion>
    </div>
  );
}
