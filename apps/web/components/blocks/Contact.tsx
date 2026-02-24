import React from "react";

import Link from "next/link";

import { Facebook, Linkedin, Twitter } from "lucide-react";

import { ContactForm } from "@/components/blocks/ContactForm";
import { DashedLine } from "@/components/DashedLine";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";

const contactInfo = [
  {
    title: "Corporate office",
    content: (
      <Text color="muted" className="mt-3">
        1 Carlsberg Close
        <br />
        1260 Hillview, Australia
      </Text>
    ),
  },
  {
    title: "Email us",
    content: (
      <div className="mt-3">
        <div>
          <Text>Careers</Text>
          <Link
            href="mailto:careers@example.com"
            className="text-muted-foreground hover:text-foreground"
          >
            careers@example.com
          </Link>
        </div>
        <div className="mt-1">
          <Text>Press</Text>
          <Link
            href="mailto:press@example.com"
            className="text-muted-foreground hover:text-foreground"
          >
            press@example.com
          </Link>
        </div>
      </div>
    ),
  },
  {
    title: "Follow us",
    content: (
      <div className="mt-3 flex gap-6 lg:gap-10">
        <Link href="#" className="text-muted-foreground hover:text-foreground">
          <Facebook className="size-5" />
        </Link>
        <Link
          href="https://x.com/ausrobdev"
          className="text-muted-foreground hover:text-foreground"
        >
          <Twitter className="size-5" />
        </Link>
        <Link href="#" className="text-muted-foreground hover:text-foreground">
          <Linkedin className="size-5" />
        </Link>
      </div>
    ),
  },
];

export default function Contact() {
  return (
    <section className="py-28 lg:py-32 lg:pt-44">
      <Container size="sm">
        <Heading level={1} className="text-center">
          Contact us
        </Heading>
        <Text color="muted" leading="snug" weight="medium" className="mt-4 text-center lg:mx-auto">
          Hopefully this form gets through our spam filters.
        </Text>

        <div className="mt-10 flex justify-between gap-8 max-sm:flex-col md:mt-14 lg:mt-20 lg:gap-12">
          {contactInfo.map((info, index) => (
            <div key={index}>
              <Text as="h2" weight="medium">{info.title}</Text>
              {info.content}
            </div>
          ))}
        </div>

        <DashedLine className="my-12" />

        {/* Inquiry Form */}
        <div className="mx-auto">
          <Heading level={2} size="xs" className="mb-4">Inquiries</Heading>
          <ContactForm />
        </div>
      </Container>
    </section>
  );
}
