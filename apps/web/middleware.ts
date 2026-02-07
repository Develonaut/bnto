import {
  authMiddleware,
  createRouteMatcher,
  middlewareRedirect,
} from "@bnto/auth/middleware";

const isProtectedRoute = createRouteMatcher(["/dashboard(.*)"]);

export default authMiddleware(async (request, { convexAuth }) => {
  if (isProtectedRoute(request)) {
    const isAuthed = await convexAuth.isAuthenticated();
    if (!isAuthed) {
      return middlewareRedirect(request, "/sign-in");
    }
  }
});

export const config = {
  matcher: ["/((?!_next/static|_next/image|favicon.ico).*)"],
};
