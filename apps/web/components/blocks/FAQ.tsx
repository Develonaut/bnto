"use client";

import Link from "next/link";

import { Accordion } from "@/components/ui/accordion";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { cn } from "@/lib/utils";

const categories = [
  {
    title: "Support",
    questions: [
      {
        question: "How do I update my account without breaking my laptop?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
      {
        question: "Is support free, or do I need to Google everything?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
      {
        question: "Are you going to be subsumed by AI?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
    ],
  },
  {
    title: "Your account",
    questions: [
      {
        question: "Is support free, or do I need to Google everything?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
      {
        question: "Are you going to be subsumed by AI?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
    ],
  },
  {
    title: "Other questions",
    questions: [
      {
        question: "Is support free, or do I need to Google everything?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
      {
        question: "Are you going to be subsumed by AI?",
        answer:
          "Lorem ipsum dolor sit amet consectetur adipisicing elit. Minus voluptates deserunt officia temporibus dignissimos.",
      },
    ],
  },
];

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
        <div className={cn("mx-auto grid gap-16 lg:grid-cols-2", className2)}>
          <Stack gap="md">
            <Heading level={headerTag === "h1" ? 1 : 2}>
              Got Questions?
            </Heading>
            <Text color="muted" leading="snug" className="max-w-md lg:mx-auto">
              If you can&apos;t find what you&apos;re looking for,{" "}
              <Link href="/contact" className="underline underline-offset-4">
                get in touch
              </Link>
              .
            </Text>
          </Stack>

          <div className="grid gap-6 text-start">
            {categories.map((category, categoryIndex) => (
              <div key={category.title} className="">
                <Text as="h3" color="muted" className="border-b py-4">
                  {category.title}
                </Text>
                <Accordion type="single" collapsible className="w-full">
                  {category.questions.map((item, i) => (
                    <Accordion.Item key={i} value={`${categoryIndex}-${i}`}>
                      <Accordion.Trigger>{item.question}</Accordion.Trigger>
                      <Accordion.Content className="text-muted-foreground">
                        {item.answer}
                      </Accordion.Content>
                    </Accordion.Item>
                  ))}
                </Accordion>
              </div>
            ))}
          </div>
        </div>
      </Container>
    </section>
  );
};
