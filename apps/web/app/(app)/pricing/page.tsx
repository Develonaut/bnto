import React from "react";

import { Background } from "#components/Background";
import { Pricing } from "#components/blocks/Pricing";
import { PricingTable } from "#components/blocks/PricingTable";

const Page = () => {
  return (
    <Background>
      <Pricing className="py-28 text-center lg:pt-44 lg:pb-32" />
      <PricingTable />
    </Background>
  );
};

export default Page;
