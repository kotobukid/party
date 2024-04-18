export type EVENT_FORMAT_ANY = 0;
export type EVENT_FORMAT_ALLSTAR = 1;
export type EVENT_FORMAT_KEYSELECTION = 2;
export type EVENT_FORMAT_DIVASELECTION = 3;
export type EVENT_FORMAT_SHIROMADO = 4;

export type EVENT_FORMAT = EVENT_FORMAT_ANY
    | EVENT_FORMAT_ALLSTAR
    | EVENT_FORMAT_KEYSELECTION
    | EVENT_FORMAT_DIVASELECTION
    | EVENT_FORMAT_SHIROMADO;

export const EventFormatAny = 0;
export const EventFormatAllStar = 1;
export const EventFormatKeySelection = 2;
export const EventFormatDivaSelection = 3;
export const EventFormatShiromado = 4;


export const PartyTypeAny: EVENT_TYPE_ANY = 0;
export const PartyTypeRegular: EVENT_TYPE_REGULAR = 1;
export const PartyTypeSpecial: EVENT_TYPE_SPECIAL = 2;

export type EVENT_TYPE_ANY = 0;
export type EVENT_TYPE_REGULAR = 1;
export type EVENT_TYPE_SPECIAL = 2;
export type EVENT_TYPE = EVENT_TYPE_ANY
    | EVENT_TYPE_REGULAR
    | EVENT_TYPE_SPECIAL;
