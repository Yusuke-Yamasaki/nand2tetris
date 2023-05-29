use super::adder::add16;
use super::basic_logic::and16;
use super::basic_logic::not16;
use super::basic_logic::Word;
use super::combinational_logic::mux16;

pub fn alu(
    a: Word,
    b: Word,
    za: bool,
    na: bool,
    zb: bool,
    nb: bool,
    f: bool,
    no: bool,
) -> (Word, bool, bool) {
    (
        mux16(
            mux16(
                and16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                add16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                f,
            ),
            not16(mux16(
                and16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                add16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                f,
            )),
            no,
        ),
        [false; 16]
            == mux16(
                mux16(
                    and16(
                        mux16(
                            mux16(a, [false; 16], za),
                            not16(mux16(a, [false; 16], za)),
                            na,
                        ),
                        mux16(
                            mux16(b, [false; 16], zb),
                            not16(mux16(b, [false; 16], zb)),
                            nb,
                        ),
                    ),
                    add16(
                        mux16(
                            mux16(a, [false; 16], za),
                            not16(mux16(a, [false; 16], za)),
                            na,
                        ),
                        mux16(
                            mux16(b, [false; 16], zb),
                            not16(mux16(b, [false; 16], zb)),
                            nb,
                        ),
                    ),
                    f,
                ),
                not16(mux16(
                    and16(
                        mux16(
                            mux16(a, [false; 16], za),
                            not16(mux16(a, [false; 16], za)),
                            na,
                        ),
                        mux16(
                            mux16(b, [false; 16], zb),
                            not16(mux16(b, [false; 16], zb)),
                            nb,
                        ),
                    ),
                    add16(
                        mux16(
                            mux16(a, [false; 16], za),
                            not16(mux16(a, [false; 16], za)),
                            na,
                        ),
                        mux16(
                            mux16(b, [false; 16], zb),
                            not16(mux16(b, [false; 16], zb)),
                            nb,
                        ),
                    ),
                    f,
                )),
                no,
            ),
        mux16(
            mux16(
                and16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                add16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                f,
            ),
            not16(mux16(
                and16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                add16(
                    mux16(
                        mux16(a, [false; 16], za),
                        not16(mux16(a, [false; 16], za)),
                        na,
                    ),
                    mux16(
                        mux16(b, [false; 16], zb),
                        not16(mux16(b, [false; 16], zb)),
                        nb,
                    ),
                ),
                f,
            )),
            no,
        )[15]
            == true,
    )
}
