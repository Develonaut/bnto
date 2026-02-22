import { ArrowUpRightIcon } from "@bnto/ui";
import { Accordion } from "@bnto/ui/accordion";

export function FAQ() {
  return (
    <section className="container grid gap-10 py-12 lg:grid-cols-3 lg:gap-16 lg:py-16">
      {/* Left column: heading + support link */}
      <div className="flex flex-col gap-4 lg:col-span-1">
        <h2 className="text-2xl tracking-tight md:text-4xl lg:text-5xl">
          Frequently asked questions
        </h2>
        <p className="text-muted-foreground text-sm">
          Can&apos;t find what you&apos;re looking for?{" "}
          <a
            href="https://github.com/Develonaut/bnto/issues"
            target="_blank"
            rel="noopener noreferrer"
            className="text-primary inline-flex items-center gap-0.5 font-medium hover:underline"
          >
            Open an issue
            <ArrowUpRightIcon className="size-3.5" />
          </a>
        </p>
      </div>

      {/* Right column: categorized accordions */}
      <div className="flex flex-col gap-8 lg:col-span-2">
        <FaqCategory label="General">
          <FaqItem
            question="What is bnto?"
            answer="bnto is a collection of free online tools for everyday tasks — compress images, clean CSVs, rename files, convert formats, and more. Every tool runs directly in your browser."
          />
          <FaqItem
            question="Is bnto free?"
            answer="Yes, completely free. No hidden fees, no premium tiers, no usage limits. The project is open source under the MIT license."
          />
          <FaqItem
            question="Do I need an account?"
            answer="No. Every tool works without signing up. An optional account unlocks features like saving workflows and execution history, but the tools themselves are always free and accessible."
          />
        </FaqCategory>

        <FaqCategory label="Privacy & Security">
          <FaqItem
            question="Are my files uploaded to a server?"
            answer="No. All processing happens locally in your browser. Your files never leave your device — there are no server uploads, no cloud storage, and no third-party access."
          />
          <FaqItem
            question="Is bnto open source?"
            answer="Yes. The entire codebase is open source under the MIT license. You can inspect the code, contribute, or self-host. No black boxes."
          />
        </FaqCategory>

        <FaqCategory label="Technical">
          <FaqItem
            question="What file types are supported?"
            answer="It depends on the tool. Image tools support PNG, JPEG, WebP, and GIF. CSV tools handle standard comma-separated files. Each tool page lists its supported formats."
          />
          <FaqItem
            question="Can I use bnto offline?"
            answer="Not yet, but a desktop app is coming. It will run the same tools natively on your machine without needing a browser or internet connection."
          />
        </FaqCategory>
      </div>
    </section>
  );
}

function FaqCategory({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div>
      <h3 className="text-muted-foreground">
        {label}
      </h3>
      <Accordion type="single" collapsible>
        {children}
      </Accordion>
    </div>
  );
}

function FaqItem({ question, answer }: { question: string; answer: string }) {
  return (
    <Accordion.Item value={question}>
      <Accordion.Trigger>{question}</Accordion.Trigger>
      <Accordion.Content>
        <p className="text-muted-foreground">{answer}</p>
      </Accordion.Content>
    </Accordion.Item>
  );
}
