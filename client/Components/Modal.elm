module Components.Modal exposing (..)

import Animation exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Time
import Color
import Messages


type alias Model =
    { animation : Animation.State
    , contents : List (Html Messages.Msg)
    , attrs : List (Attribute Messages.Msg)
    }


create : List (Attribute Messages.Msg) -> List (Html Messages.Msg) -> Model
create attrs contents =
    { animation = Animation.interrupt dialogAppear dialogAppearStyle
    , contents = contents
    , attrs = attrs
    }


view : Model -> Html Messages.Msg
view model =
    div (List.concat [ model.attrs, Animation.render model.animation, [ class "dialog" ] ])
        (List.append model.contents [ button [ type_ "button" ] [ text "OK" ] ])


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
