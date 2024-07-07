struct Ok
{
    int x;
};

Ok TestOk()
{
    Ok r;
    r.x = 1;
    return r;
}

////////////////////////////

struct Err
{
    Err() {} // <--- Difference here!!!
    int x;
};

Err TestErr()
{
    Err r;
    r.x = 1;
    return r;
}

