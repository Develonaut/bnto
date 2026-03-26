import { MoonIcon, SunIcon, SunsetIcon } from "../icons";

/** Three theme icons that transition based on the parent's data-theme attribute. */
export function ThemeIcons() {
  return (
    <>
      <SunIcon className="absolute size-4 scale-0 rotate-90 transition-all group-data-[theme=light]:scale-100 group-data-[theme=light]:rotate-0" />
      <SunsetIcon className="absolute size-4 scale-0 rotate-90 transition-all group-data-[theme=sunset]:scale-100 group-data-[theme=sunset]:rotate-0" />
      <MoonIcon className="absolute size-4 scale-0 -rotate-90 transition-all group-data-[theme=dark]:scale-100 group-data-[theme=dark]:rotate-0" />
      <span className="sr-only">Toggle theme</span>
    </>
  );
}
