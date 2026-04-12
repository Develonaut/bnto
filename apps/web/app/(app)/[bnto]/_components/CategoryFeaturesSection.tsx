"use client";

import { Card, Heading, InView, SlideUp, Stagger, ScaleIn, Stack, Text } from "@bnto/ui";
import { getFeatures } from "@/lib/recipePageContent";
import { getCategoryMascot } from "../_utils/categoryMascot";

export function CategoryFeaturesSection({ category }: { category: string }) {
  const features = getFeatures(category);
  if (features.length === 0) return null;

  return (
    <InView>
      <div className="grid items-start gap-12 lg:grid-cols-[3fr_2fr] lg:gap-20">
        <Stagger className="space-y-4">
          {features.map((feature, i) => (
            <ScaleIn key={i} index={i} from={0.9}>
              <Card elevation="sm" className="p-5">
                <Heading level={3} size="sm" className="mb-1">
                  {feature.heading}
                </Heading>
                <Text color="muted" leading="snug" size="sm">
                  {feature.body}
                </Text>
              </Card>
            </ScaleIn>
          ))}
        </Stagger>
        <SlideUp>
          <Stack gap="md">
            {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
            <img
              src={getCategoryMascot(category)}
              alt=""
              className="w-auto self-start"
              style={{ height: 140 }}
              aria-hidden
            />
            <Heading level={2} size="xl">
              Built for real workflows
            </Heading>
            <Text color="muted" leading="snug">
              Every feature is designed around how people actually work: batch processing, format
              flexibility, and zero configuration. Drop your files and go.
            </Text>
          </Stack>
        </SlideUp>
      </div>
    </InView>
  );
}
