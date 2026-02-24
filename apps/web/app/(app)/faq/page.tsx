import React from "react";

import { Background } from "@/components/Background";
import { FAQ } from "@/components/blocks/FAQ";
import { Testimonials } from "@/components/blocks/Testimonials";
import { DashedLine } from "@/components/DashedLine";

const Page = () => {
  return (
    <Background>
      <FAQ
        className="py-28 text-center lg:pt-44 lg:pb-32"
        className2="max-w-xl lg:grid-cols-1"
        headerTag="h1"
      />
      <DashedLine className="mx-auto max-w-xl" />
      <Testimonials dashedLineClassName="hidden" />
    </Background>
  );
};

export default Page;
