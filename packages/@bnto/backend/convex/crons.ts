import { cronJobs } from "convex/server";
import { internal } from "./_generated/api";

const crons = cronJobs();

// Catch executions stuck in pending/running for >2 hours.
// Likely caused by Go API crash, network drop, or polling timeout edge case.
crons.interval(
  "cleanup stale executions",
  { hours: 1 },
  internal.cleanup_stale.cleanupStaleExecutions,
);

export default crons;
