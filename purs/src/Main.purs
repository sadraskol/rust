module Main where

import Prelude

import AstComponent as AstComponent
import Effect (Effect)
import Halogen.Aff as HA
import Halogen.VDom.Driver (runUI)

main :: Effect Unit
main = HA.runHalogenAff do
  body <- HA.awaitBody

  runUI AstComponent.component unit body
