import { Container } from "@/components/ui/Container";

import { ImageSection } from "./ImageSection";
import { TextSection } from "./TextSection";

const About = () => {
  return (
    <Container as="section" size="md" className="mt-10 flex flex-col-reverse gap-8 md:mt-14 md:gap-14 lg:mt-20 lg:flex-row lg:items-end">
      {/* Images Left - Text Right */}
      <div className="flex flex-col gap-8 lg:gap-16 xl:gap-20">
        <ImageSection
          images={[
            { src: "/about/1.webp", alt: "Team collaboration" },
            { src: "/about/2.webp", alt: "Team workspace" },
          ]}
          className="xl:-translate-x-10"
        />

        <TextSection
          title="The team"
          paragraphs={[
            "We started building Mainline in 2019 and launched in 2022. Every endpoint has been designed from the ground up — with no technical debt or legacy systems. We are purpose-built to power project management innovation for the next hundred years.",
            "We are 100% founder and team-owned, profitable, and we keep our team lean. Over time, this page will become more polished, but for now, we're focused on delivering for developers.",
            "If you're interested in building the future of PM, check out our open roles below.",
          ]}
          ctaButton={{
            href: "/careers",
            text: "View open roles",
          }}
        />
      </div>

      {/* Text Left - Images Right */}
      <div className="flex flex-col gap-8 lg:gap-16 xl:gap-20">
        <TextSection
          paragraphs={[
            "At Mainline, we are dedicated to transforming the way teams plan, execute, and deliver projects. Our mission is to provide our customers with an unbeatable edge over delays, inefficiencies, and disorganisation through actionable insights and seamless collaboration. We'll stop at nothing to give you the tools you need to get every project across the finish line.",
            "We're customer-obsessed — investing the time to understand every aspect of your workflow so that we can help you operate better than ever before. We're all in this together because your success is our success. In our history as a company, we've never lost a customer, because when your projects succeed, so do we.",
          ]}
        />
        <ImageSection
          images={[
            { src: "/about/3.webp", alt: "Modern workspace" },
            { src: "/about/4.webp", alt: "Team collaboration" },
          ]}
          className="hidden lg:flex xl:translate-x-10"
        />
      </div>
    </Container>
  );
};

export default About;
