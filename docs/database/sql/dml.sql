BEGIN
TRANSACTION;

INSERT INTO
    users (id, username, hash, salt)
VALUES
    (
        1,
        'server',
        '052a399f2bda24505aeac6257e168dd7',
        '16fc5a99e1df3d38fad30428cfd802f4'
    ),
    (
        2,
        'mario',
        '1db40a70bd8bce4814af5b9de356ed81',
        'f805ac69bbac7deb853d187df2a33fe5'
    ),
    (
        3,
        'luigi',
        '0c12ac8d8be294290db7ad69dde1c220',
        'f75b69be13aeb0ee020c63c4fd612747'
    ),
    (
        4,
        'ugo',
        'c7b9f90085264b516219d6a0a51a44f3',
        '3c8308e9c47102a4aeb4d57199346001'
    );

COMMIT;
