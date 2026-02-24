import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { DashedLine } from "@/components/dashed-line";

const stats = [
  {
    value: "$150M",
    label: "Raised",
  },
  {
    value: "20K",
    label: "Companies",
  },
  {
    value: "1.3B",
    label: "Monthly transactions",
  },
  {
    value: "1.5K",
    label: "Connections per minute",
  },
];

export function AboutHero() {
  return (
    <section className="">
      <Container size="md" className="flex flex-col justify-between gap-8 md:gap-20 lg:flex-row lg:items-center lg:gap-24 xl:gap-24">
        <div className="flex-[1.5]">
          <Heading level={1}>
            Democratising quality software
          </Heading>

          <Text color="muted" className="mt-5 text-2xl md:text-3xl lg:text-4xl">
            Mainline is bringing modern software to life with AI magic.
          </Text>

          <Text color="muted" size="lg" balance className="mt-8 hidden max-w-lg space-y-6 md:block lg:mt-12">
            At Mainline, we are dedicated to transforming the way teams plan,
            execute, and deliver projects. Our mission is to provide our
            customers with an unbeatable edge over delays, inefficiencies, and
            disorganisation through actionable insights and seamless
            collaboration. We&apos;ll stop at nothing to give you the tools you
            need to get every project across the finish line.
            <br />
            <br />
            We&apos;re customer-obsessed — investing the time to understand
            every aspect of your workflow so that we can help you operate better
            than ever before. We&apos;re all in this together because your
            success is our success. In our history as a company, we&apos;ve
            never lost a customer, because when your projects succeed, so do we.
          </Text>
        </div>

        <div
          className={`relative flex flex-1 flex-col justify-center gap-3 pt-10 lg:pt-0 lg:pl-10`}
        >
          <DashedLine
            orientation="vertical"
            className="absolute top-0 left-0 max-lg:hidden"
          />
          <DashedLine
            orientation="horizontal"
            className="absolute top-0 lg:hidden"
          />
          {stats.map((stat) => (
            <Stack key={stat.label} gap="xs">
              <div className="font-display text-4xl tracking-wide md:text-5xl">
                {stat.value}
              </div>
              <Text color="muted" as="span">{stat.label}</Text>
            </Stack>
          ))}
        </div>
      </Container>
    </section>
  );
}
