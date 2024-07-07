struct VecYYY
{
    int x;
};

VecYYY TestYYY();

VecYYY TestYYY()
{
    VecYYY r;
    r.x = 1;
    return r;
}

////////////////////////////

struct VecXXX
{
    VecXXX() {}
    int x;
};

VecXXX TestXXX();

VecXXX TestXXX()
{
    VecXXX r;
    r.x = 1;
    return r;
}

