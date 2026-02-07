import { ConvexAuthNextjsServerProvider } from "@convex-dev/auth/nextjs/server";

export function BntoAuthProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <ConvexAuthNextjsServerProvider>
      {children}
    </ConvexAuthNextjsServerProvider>
  );
}
