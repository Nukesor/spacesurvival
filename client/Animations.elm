module Animations exposing (..)

import Animation exposing (..)
import Time


wiggle =
    [ loop
        [ to [ translate (px 100) (px 0) ]
        , to [ translate (px 50) (px 0) ]
        , to [ translate (px 500) (px 0)]
        ]
    ]


flicker =
    [ loop
        [ set [ opacity 0.5 ]
        , wait <| Time.second
        , set [ opacity 1.0 ]
        ]
    ]
