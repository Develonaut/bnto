import Link from "next/link";

import { Button } from "@/components/ui/button";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

interface TextSectionProps {
  title?: string;
  paragraphs: string[];
  ctaButton?: {
    href: string;
    text: string;
  };
}

export function TextSection({
  title,
  paragraphs,
  ctaButton,
}: TextSectionProps) {
  return (
    <Stack as="section" gap="md" className="flex-1 text-lg">
      {title && <Heading level={2} className="text-4xl">{title}</Heading>}
      <Stack className="gap-6 max-w-xl">
        {paragraphs.map((paragraph, index) => (
          <Text key={index} color="muted">{paragraph}</Text>
        ))}
      </Stack>
      {ctaButton && (
        <div className="mt-8">
          <Link href={ctaButton.href}>
            <Button size="lg">{ctaButton.text}</Button>
          </Link>
        </div>
      )}
    </Stack>
  );
}
