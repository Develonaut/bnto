"use client";

import { Accordion } from "@/components/ui/accordion";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { GITHUB_URL } from "@/lib/copy";
import { cn } from "@/lib/utils";

const faqs = [
  {
    question: "Where are my files processed?",
    answer:
      "Entirely in your browser. bnto uses Rust compiled to WebAssembly — your files are processed on your device and never uploaded to a server. Nothing leaves your machine.",
  },
  {
    question: "Is it really free?",
    answer:
      "Yes. Every tool that runs in your browser is free, unlimited, forever. No signup, no watermarks, no quality reduction, no daily caps. We will never put limits on browser-based tools.",
  },
  {
    question: "What file types are supported?",
    answer:
      "Images: JPEG, PNG, and WebP for compression, resizing, and format conversion. Data: CSV files for cleaning and column renaming. Files: any file type for batch renaming. More formats are on the way.",
  },
  {
    question: "Is bnto open source?",
    answer:
      "Yes. The entire codebase is MIT licensed. You can inspect the source code, verify what happens to your files, or contribute. Nothing is hidden.",
  },
  {
    question: "What is Pro?",
    answer:
      "Pro ($8/month) unlocks features that require a server: AI-powered tools, saved recipes, execution history, team sharing, and API access. Browser tools stay free — Pro is for power users who want persistence and premium compute.",
  },
  {
    question: "Do I need an account?",
    answer:
      "No. Drop your files and use any browser tool immediately. Accounts are only needed if you want to save recipes or access Pro features.",
  },
  {
    question: "Can I process multiple files at once?",
    answer:
      "Yes. Drop as many files as you want — bnto processes them all in a single batch. Results download as a ZIP when there are multiple output files.",
  },
] as const;

export const FAQ = ({
  headerTag = "h2",
  className,
  className2,
}: {
  headerTag?: "h1" | "h2";
  className?: string;
  className2?: string;
}) => {
  return (
    <section className={cn("py-28 lg:py-32", className)}>
      <Container size="md">
        <div className={cn("mx-auto grid gap-12 lg:grid-cols-2 lg:gap-16", className2)}>
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
      </Container>
    </section>
  );
};
