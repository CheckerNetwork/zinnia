import { core, primordials } from "ext:core/mod.js";
const { ObjectCreate, ObjectDefineProperties } = primordials;

import { op_info_activity, op_error_activity, op_job_completed, op_zinnia_log } from "ext:core/ops";

import { inspect } from "ext:deno_console/01_console.js";
import { versions } from "ext:zinnia_runtime/01_version.ts";

const zinniaNs = ObjectCreate(null);

const activityApi = ObjectCreate(null);
ObjectDefineProperties(activityApi, {
  info: core.propReadOnly(reportInfoActivity),
  error: core.propReadOnly(reportErrorActivity),
});

ObjectDefineProperties(zinniaNs, {
  activity: core.propReadOnly(activityApi),
  jobCompleted: core.propReadOnly(reportJobCompleted),
  versions: core.propReadOnly(versions),
  inspect: core.propReadOnly(inspect),
});

function reportInfoActivity(msg) {
  if (typeof msg !== "string") msg = "" + msg;
  op_info_activity(msg);
}

function reportErrorActivity(msg) {
  if (typeof msg !== "string") msg = "" + msg;
  op_error_activity(msg);
}

function reportJobCompleted() {
  op_job_completed();
}

function log(msg, level) {
  if (typeof msg !== "string") msg = "" + msg;
  op_zinnia_log(msg, level);
}

export { zinniaNs, log };
