import bionic_writer

def test_bic_writer_write_with_markdown():
    text = (
        "Now that we know who you are, I know who I am. I'm not "
        "a mistake! It all makes sense! In a comic, you know how you can tell "
        "who the arch-villain's going to be? He's the exact opposite of the "
        "hero. And most times they're friends, like you and me! I should've "
        "known way back when... You know why, David? Because of the kids. They "
        "called me Mr Glass."
    )
    # Mardown bold emphasis
    affix = postfix = "**"

    result = bionic_writer.write(text=text, affix=affix, postfix=postfix)

    expected = (
        "**N**ow **th**at **w**e **kn**ow **w**ho **y**ou **a**re, **I** "
        "**kn**ow **w**ho **I** **a**m. **I**'**m** **n**ot **a** "
        "**mis**take! **I**t **a**ll **ma**kes **se**nse! **I**n **a** "
        "**co**mic, **y**ou **kn**ow **h**ow **y**ou **c**an **te**ll **w**ho "
        "**t**he **ar**ch-**vil**lain'**s** **go**ing **t**o **b**e? "
        "**H**e'**s** **t**he **ex**act **oppo**site **o**f **t**he **he**ro. "
        "**A**nd **mo**st **ti**mes **th**ey'**r**e **fri**ends, **li**ke "
        "**y**ou **a**nd **m**e! **I** **sho**uld'**v**e **kn**own **w**ay "
        "**ba**ck **wh**en... **Y**ou **kn**ow **w**hy, **Da**vid? "
        "**Bec**ause **o**f **t**he **ki**ds. **Th**ey **cal**led **m**e "
        "**M**r **Gl**ass."
    );
    assert expected == result

# Rust version
#
# Move to .rs file and Add "rlib" to crate-type.
#
# #[test]
# fn write_text_as_bionic_reading() {
#     let text = concat!(
#         "Now that we know who you are, I know who I am. I'm not ",
#         "a mistake! It all makes sense! In a comic, you know how you can tell ",
#         "who the arch-villain's going to be? He's the exact opposite of the ",
#         "hero. And most times they're friends, like you and me! I should've ",
#         "known way back when... You know why, David? Because of the kids. They ",
#         "called me Mr Glass."
#     );
#     let affix = "**";
#     let postfix = "**";

#     let result = bionic_writer::write(text, affix, postfix);

#     let expected = concat!(
#         "**N**ow **th**at **w**e **kn**ow **w**ho **y**ou **a**re, **I** ",
#         "**kn**ow **w**ho **I** **a**m. **I**'**m** **n**ot **a** ",
#         "**mis**take! **I**t **a**ll **ma**kes **se**nse! **I**n **a** ",
#         "**co**mic, **y**ou **kn**ow **h**ow **y**ou **c**an **te**ll **w**ho ",
#         "**t**he **ar**ch-**vil**lain'**s** **go**ing **t**o **b**e? ",
#         "**H**e'**s** **t**he **ex**act **oppo**site **o**f **t**he **he**ro. ",
#         "**A**nd **mo**st **ti**mes **th**ey'**r**e **fri**ends, **li**ke ",
#         "**y**ou **a**nd **m**e! **I** **sho**uld'**v**e **kn**own **w**ay ",
#         "**ba**ck **wh**en... **Y**ou **kn**ow **w**hy, **Da**vid? ",
#         "**Bec**ause **o**f **t**he **ki**ds. **Th**ey **cal**led **m**e ",
#         "**M**r **Gl**ass."
#     );
#     assert_eq!(expected, result.unwrap());
# }
