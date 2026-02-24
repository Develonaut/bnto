"use client";

import { useState } from "react";

import { Check } from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { Switch } from "@/components/ui/switch";
import { cn } from "@/lib/utils";

const plans = [
  {
    name: "Free",
    monthlyPrice: "$0",
    yearlyPrice: "$0",
    description: "Free for everyone",
    features: [
      "Unlimited members",
      "2 teams",
      "500 issues",
      "Slack and Github integrations",
    ],
  },
  {
    name: "Startup",
    monthlyPrice: "$8",
    yearlyPrice: "$6",
    features: [
      "All free plan features and...",
      "Mainline AI",
      "Unlimited teams",
      "Unlimited issues and file uploads",
      "Mainline Insights",
      "Admin roles",
    ],
  },
  {
    name: "Enterprise",
    monthlyPrice: "$8",
    yearlyPrice: "$6",
    features: [
      "All free plan features and...",
      "Mainline AI",
      "Supermainline AGI",
      "Free daily catered lunch",
      "random HIPPA audits",
    ],
  },
];

export const Pricing = ({ className }: { className?: string }) => {
  const [isAnnual, setIsAnnual] = useState(true);

  return (
    <section className={cn("py-28 lg:py-32", className)}>
      <Container size="md">
        <Stack gap="md" className="text-center">
          <Heading level={2}>
            Pricing
          </Heading>
          <Text color="muted" leading="snug" balance className="mx-auto max-w-xl">
            Use Mainline for free with your whole team. Upgrade to enable
            unlimited issues, enhanced security controls, and additional
            features.
          </Text>
        </Stack>

        <div className="mt-8 grid items-start gap-5 text-start md:mt-12 md:grid-cols-3 lg:mt-20">
          {plans.map((plan) => (
            <Card
              key={plan.name}
              className={`${
                plan.name === "Startup"
                  ? "outline-primary origin-top outline-4"
                  : ""
              }`}
            >
              <Card.Content className="flex flex-col gap-7 px-6 py-5">
                <Stack gap="sm">
                  <Heading level={3} size="xs" className="text-lg">{plan.name}</Heading>
                  <Stack gap="xs">
                    <div className="text-muted-foreground text-lg font-medium">
                      {isAnnual ? plan.yearlyPrice : plan.monthlyPrice}{" "}
                      {plan.name !== "Free" && (
                        <span className="text-muted-foreground">
                          per user/
                          {isAnnual ? "year" : "month"}
                        </span>
                      )}
                    </div>
                  </Stack>
                </Stack>

                {plan.name !== "Free" ? (
                  <div className="flex items-center gap-2">
                    <Switch
                      checked={isAnnual}
                      onCheckedChange={() => setIsAnnual(!isAnnual)}
                      aria-label="Toggle annual billing"
                    />
                    <span className="text-sm font-medium">Billed annually</span>
                  </div>
                ) : (
                  <span className="text-muted-foreground text-sm">
                    {plan.description}
                  </span>
                )}

                <Stack className="gap-3">
                  {plan.features.map((feature) => (
                    <div
                      key={feature}
                      className="text-muted-foreground flex items-center gap-1.5"
                    >
                      <Check className="size-5 shrink-0" />
                      <span className="text-sm">{feature}</span>
                    </div>
                  ))}
                </Stack>

                <Button
                  className="w-fit"
                  variant={plan.name === "Startup" ? "default" : "outline"}
                >
                  Get started
                </Button>
              </Card.Content>
            </Card>
          ))}
        </div>
      </Container>
    </section>
  );
};
