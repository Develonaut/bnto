import Image from "next/image";

import {
  ArrowRight,
  Blend,
  ChartNoAxesColumn,
  CircleDot,
  Diamond,
} from "lucide-react";

import { DashedLine } from "@/components/DashedLine";
import { Button } from "@/components/ui/button";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Row } from "@/components/ui/Row";

const features = [
  {
    title: "Tailored workflows",
    description: "Track progress across custom issue flows for your team.",
    icon: CircleDot,
  },
  {
    title: "Cross-team projects",
    description: "Collaborate across teams and departments.",
    icon: Blend,
  },
  {
    title: "Milestones",
    description: "Break projects down into concrete phases.",
    icon: Diamond,
  },
  {
    title: "Progress insights",
    description: "Track scope, velocity, and progress over time.",
    icon: ChartNoAxesColumn,
  },
];

export const Hero = () => {
  return (
    <section className="py-28 lg:py-32 lg:pt-44">
      <Container className="flex flex-col justify-between gap-8 md:gap-14 lg:flex-row lg:gap-20">
        {/* Left side - Main content */}
        <div className="flex-1">
          <Heading level={1} className="max-w-160 xl:whitespace-nowrap">
            Mainline Next.js template
          </Heading>

          <Text color="muted" className="text-1xl mt-5 md:text-3xl">
            Mainline is an open-source website template built with shadcn/ui,
            Tailwind 4 & Next.js
          </Text>

          <Row wrap className="mt-8 gap-4 lg:flex-nowrap">
            <Button asChild>
              <a href="https://github.com/shadcnblocks/mainline-nextjs-template">
                Get template
              </a>
            </Button>
            <Button
              variant="outline"
              className="from-background h-auto gap-2 bg-linear-to-r to-transparent shadow-md"
              asChild
            >
              <a
                href="https://shadcnblocks.com"
                className="max-w-56 truncate text-start md:max-w-none"
              >
                Built by shadcnblocks.com
                <ArrowRight className="stroke-3" />
              </a>
            </Button>
          </Row>
        </div>

        {/* Right side - Features */}
        <div className="relative flex flex-1 flex-col justify-center space-y-5 max-lg:pt-10 lg:pl-10">
          <DashedLine
            orientation="vertical"
            className="absolute top-0 left-0 max-lg:hidden"
          />
          <DashedLine
            orientation="horizontal"
            className="absolute top-0 lg:hidden"
          />
          {features.map((feature) => {
            const Icon = feature.icon;
            return (
              <div key={feature.title} className="flex gap-2.5 lg:gap-5">
                <Icon className="text-foreground mt-1 size-4 shrink-0 lg:size-5" />
                <div>
                  <Heading level={2} size="xs">
                    {feature.title}
                  </Heading>
                  <Text size="sm" color="muted" className="max-w-76">
                    {feature.description}
                  </Text>
                </div>
              </div>
            );
          })}
        </div>
      </Container>

      <Container className="mt-12 max-lg:mx-0 max-lg:px-0 max-lg:ml-6 max-lg:h-[550px] max-lg:overflow-hidden md:mt-20 lg:mt-24">
        <div className="relative h-[793px] w-full">
          <Image
            src="/hero.webp"
            alt="hero"
            fill
            className="rounded-2xl object-cover object-left-top shadow-lg max-lg:rounded-tr-none"
          />
        </div>
      </Container>
    </section>
  );
};
