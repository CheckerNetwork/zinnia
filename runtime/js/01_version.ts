// Copyright 2018-2025 the Deno authors. MIT license.

import { primordials } from "ext:core/mod.js";
const { ObjectFreeze } = primordials;

const versions = {
  zinnia: "",
  v8: "",
};

function setVersions(zinniaVersion, v8Version) {
  versions.zinnia = zinniaVersion;
  versions.v8 = v8Version;

  ObjectFreeze(versions);
}

export { setVersions, versions };
