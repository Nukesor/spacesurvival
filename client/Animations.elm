module Animations exposing (..)

import Animation exposing (..)
import Time
import Color


wiggle =
    [ loop
        [ to [ translate (px 100) (px 0) ]
        , to [ translate (px 50) (px 0) ]
        , to [ translate (px 500) (px 0) ]
        ]
    ]


flicker =
    [ loop
        [ set [ opacity 0.5 ]
        , wait <| Time.second
        , set [ opacity 1.0 ]
        ]
    ]


dialogAppearStyle =
    styleWith
        (Animation.spring { stiffness = 150, damping = 30 })
        [ translate (px 0) (percent -300), scale3d 0.01 0.01 1, backgroundColor Color.white ]


dialogAppear =
    [ to [ translate (px 0) (px 0) ]
    , to [ scale3d 1 0.01 1 ]
    , wait <| Time.second * 0.5
    , to [ scale3d 1 1 1, backgroundColor <| Color.rgba 0 0 0 0.0 ]
    ]
