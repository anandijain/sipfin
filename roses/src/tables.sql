/*
schemas

*/

create table rt(
	symbol varchar(16),
	t timestamp, 
	x float,
	v int
);

create table fred(
	t date,
	x float,
	id varchar(32)
);

/*
queries

*/
select a.id, a.t, a.x, b.id, b.t, b.x from fred a inner join fred b on a.t = b.t where b.id = 'DGS3MO' and a.id = 'DPRIME';

select a.t, a.x x1, b.x x2, c.x x3 from fred a inner join fred b on a.t = b.t inner join fred c on b.t = c.t where a.id = 'DPRIME' and b.id = 'DGS3MO' and c.id = 'DTB6';

