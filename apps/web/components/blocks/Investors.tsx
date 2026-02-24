import Image from "next/image";

import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";

const investors = [
  {
    name: "Dennis Bouvard",
    company: "Blackbird Ventures",
    image: "/investors/1.webp",
  },
  {
    name: "Renatus Gerard",
    company: "Center Studies",
    image: "/investors/2.webp",
  },
  {
    name: "Leslie Alexander",
    company: "TechNexus",
    image: "/investors/3.webp",
  },
  {
    name: "Matthew Stephens",
    company: "Etymol Cap",
    image: "/investors/4.webp",
  },
  {
    name: "Josephine Newman",
    company: "Vandenberg",
    image: "/investors/5.webp",
  },
];

export function Investors() {
  return (
    <Container as="section" size="md" className="py-12">
      <Heading level={2} className="text-4xl font-medium tracking-wide">
        Our investors
      </Heading>
      <div className="mt-8 grid grid-cols-2 gap-12 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5">
        {investors.map((investor) => (
          <div key={investor.name} className="">
            <Image
              src={investor.image}
              alt={investor.name}
              width={120}
              height={120}
              className="object-cover"
            />
            <Heading level={3} size="xs" className="mt-3 text-base">{investor.name}</Heading>
            <Text color="muted">{investor.company}</Text>
          </div>
        ))}
      </div>
    </Container>
  );
}
