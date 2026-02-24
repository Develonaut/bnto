import { Background } from "@/components/Background";
import { BntoGallery } from "@/components/blocks/BntoGallery";
import { FAQ } from "@/components/blocks/FAQ";
import { Features } from "@/components/blocks/Features";
import { Hero } from "@/components/blocks/Hero";
import { Logos } from "@/components/blocks/Logos";
import { Pricing } from "@/components/blocks/Pricing";
import { ResourceAllocation } from "@/components/blocks/ResourceAllocation";
import { Testimonials } from "@/components/blocks/Testimonials";

export default function Home() {
  return (
    <>
      <Background className="via-muted to-muted/80">
        <Hero />
        <Logos />
        <BntoGallery />
        <Features />
        <ResourceAllocation />
      </Background>
      <Testimonials />
      <Background variant="bottom">
        <Pricing />
        <FAQ />
      </Background>
    </>
  );
}
