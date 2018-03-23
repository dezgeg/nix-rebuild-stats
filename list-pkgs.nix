{ nixpkgs }:

let
  pkgs = import nixpkgs {};
  processAttrset = attrPath: attrSet: keys:
    pkgs.lib.concatMap
      (attr:
        let e = (builtins.tryEval attrSet.${attr});
            e2 = (builtins.tryEval attrSet.${attr}.drvPath);
        in if !e.success
          then []
          else if (e.value.recurseForDerivations or false) || (e.value.recurseForRelease or false)
            then processAttrset "${attrPath}${attr}." e.value (builtins.attrNames e.value)
            else if pkgs.lib.isDerivation e.value && e2.success
              then [["${attrPath}${attr}" e.value.drvPath]]
              else [])
      keys;
in
  processAttrset "" pkgs (builtins.attrNames pkgs)
