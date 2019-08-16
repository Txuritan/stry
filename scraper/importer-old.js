"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const cheerio_1 = __importDefault(require("cheerio"));
const fs_1 = __importDefault(require("fs"));
const npmlog_1 = __importDefault(require("npmlog"));
const sync_request_1 = __importDefault(require("sync-request"));
const turndown_1 = __importDefault(require("turndown"));
var TagType;
(function (TagType) {
    TagType[TagType["Warning"] = 0] = "Warning";
    TagType[TagType["Pairing"] = 1] = "Pairing";
    TagType[TagType["Character"] = 2] = "Character";
    TagType[TagType["General"] = 3] = "General";
})(TagType || (TagType = {}));
var DlSite;
(function (DlSite) {
    DlSite[DlSite["ArchiveOfOurOwn"] = 0] = "ArchiveOfOurOwn";
    DlSite[DlSite["FanFiction"] = 1] = "FanFiction";
})(DlSite || (DlSite = {}));
function sleep(time) {
    npmlog_1.default.info("util", `Sleeping For ${time / 1000} Seconds`);
    const stop = new Date().getTime();
    while (new Date().getTime() < stop + time) { }
}
function random(low, high) {
    return Math.floor(Math.random() * (high - low + 1) + low);
}
const Service = new turndown_1.default();
const Stories = [{
        id: "12382425",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "in-progress",
        created: {
            year: 2017,
            month: 2,
            day: 25,
        },
        updated: {
            year: 2018,
            month: 4,
            day: 8,
        },
        chapters: 40,
        origins: ["Harry Potter"],
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Ron Weasley",
                type: TagType.Character,
            }, {
                name: "Percy Weasley",
                type: TagType.Character,
            }, {
                name: "Fred Weasley",
                type: TagType.Character,
            }, {
                name: "Redhead!Harry Potter",
                type: TagType.General,
            }],
    }, {
        id: "12388283",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completed",
        created: {
            year: 2017,
            month: 3,
            day: 2,
        },
        updated: {
            year: 2017,
            month: 6,
            day: 14,
        },
        origins: ["Harry Potter"],
        chapters: 78,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }],
    }, {
        id: "12238345",
        site: DlSite.FanFiction,
        rating: "mature",
        state: "completed",
        created: {
            year: 2016,
            month: 11,
            day: 18,
        },
        updated: {
            year: 2017,
            month: 8,
            day: 22,
        },
        origins: ["Harry Potter"],
        chapters: 44,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }],
    }, {
        id: "12511998",
        site: DlSite.FanFiction,
        rating: "mature",
        state: "completed",
        created: {
            year: 2017,
            month: 5,
            day: 31,
        },
        updated: {
            year: 2017,
            month: 7,
            day: 6,
        },
        origins: ["Harry Potter"],
        chapters: 19,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Bellatrix Lestrange",
                type: TagType.Character,
            }, {
                name: "Charlus Potter",
                type: TagType.Character,
            }],
    }, {
        id: "12614436",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completed",
        created: {
            year: 2017,
            month: 8,
            day: 14,
        },
        updated: {
            year: 2017,
            month: 10,
            day: 19,
        },
        origins: ["Harry Potter"],
        chapters: 11,
        tags: [{
                name: "Hermione Granger",
                type: TagType.Character,
            }, {
                name: "Theodore Nott",
                type: TagType.Character,
            }],
    }, {
        id: "12310861",
        site: DlSite.FanFiction,
        rating: "mature",
        state: "completed",
        created: {
            year: 2017,
            month: 1,
            day: 6,
        },
        updated: {
            year: 2019,
            month: 6,
            day: 6,
        },
        origins: ["Harry Potter"],
        chapters: 40,
        tags: [{
                name: "Harry Potter/Hermione Granger",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }],
    }, {
        id: "12568760",
        site: DlSite.FanFiction,
        rating: "mature",
        state: "in-progress",
        created: {
            year: 2017,
            month: 7,
            day: 11,
        },
        updated: {
            year: 2018,
            month: 5,
            day: 31,
        },
        origins: ["Harry Potter"],
        chapters: 16,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }, {
                name: "Fleur Delacour",
                type: TagType.Character,
            }, {
                name: "Albus Dumbledore",
                type: TagType.Character,
            }],
    }, {
        id: "12578431",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completef",
        created: {
            year: 2017,
            month: 7,
            day: 18,
        },
        updated: {
            year: 2017,
            month: 12,
            day: 3,
        },
        origins: ["Harry Potter"],
        chapters: 22,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Ron Weasley",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }],
    }, {
        id: "12746586",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completed",
        created: {
            year: 2017,
            month: 12,
            day: 3,
        },
        updated: {
            year: 2018,
            month: 4,
            day: 29,
        },
        origins: ["Harry Potter"],
        chapters: 24,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Ron Weasley",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }],
    }, {
        id: "12592097",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completed",
        created: {
            year: 2017,
            month: 7,
            day: 29,
        },
        updated: {
            year: 2018,
            month: 11,
            day: 3,
        },
        origins: ["Harry Potter"],
        chapters: 67,
        tags: [{
                name: "Harry Potter/Hermione Granger",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }, {
                name: "Sirius Black",
                type: TagType.Character,
            }, {
                name: "Mundungus Fletcher",
                type: TagType.Character,
            }, {
                name: "Expelled!Hermione Granger",
                type: TagType.General,
            }, {
                name: "Framed!Hermione Granger",
                type: TagType.General,
            }, {
                name: "Thief!Hermione Granger",
                type: TagType.General,
            }],
    }, {
        id: "12332867",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completed",
        created: {
            year: 2017,
            month: 1,
            day: 22,
        },
        updated: {
            year: 2017,
            month: 1,
            day: 22,
        },
        origins: ["Harry Potter"],
        chapters: 1,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Ginny Weasley",
                type: TagType.Character,
            }, {
                name: "Luna Lovegood",
                type: TagType.Character,
            }, {
                name: "Xenophilius Lovegood",
                type: TagType.Character,
            }],
    }, {
        id: "12487457",
        site: DlSite.FanFiction,
        rating: "general",
        state: "completed",
        created: {
            year: 2017,
            month: 5,
            day: 13,
        },
        updated: {
            year: 2017,
            month: 5,
            day: 13,
        },
        origins: ["Harry Potter"],
        chapters: 1,
        tags: [{
                name: "Harry Potter/Luna Lovegood",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Luna Lovegood",
                type: TagType.Character,
            }, {
                name: "Hedwig",
                type: TagType.Character,
            }],
    }, {
        id: "12499983",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "completed",
        created: {
            year: 2017,
            month: 5,
            day: 23,
        },
        updated: {
            year: 2017,
            month: 5,
            day: 23,
        },
        origins: ["Harry Potter"],
        chapters: 1,
        tags: [{
                name: "Harry Potter/Luna Lovegood",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Luna Lovegood",
                type: TagType.Character,
            }, {
                name: "Xenophilius Lovegood",
                type: TagType.Character,
            }],
    }, {
        id: "11689499",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "in-progress",
        created: {
            year: 2015,
            month: 12,
            day: 25,
        },
        updated: {
            year: 2018,
            month: 6,
            day: 15,
        },
        origins: ["Harry Potter"],
        chapters: 30,
        tags: [{
                name: "Harry Potter/Hermione Granger",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Hermione Grander",
                type: TagType.Character,
            }],
    }, {
        id: "6728900",
        site: DlSite.FanFiction,
        rating: "teen",
        state: "in-progress",
        created: {
            year: 2011,
            month: 2,
            day: 9,
        },
        updated: {
            year: 2018,
            month: 7,
            day: 3,
        },
        origins: ["Harry Potter"],
        chapters: 7,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }],
    }, {
        id: "1598530",
        site: DlSite.FanFiction,
        rating: "general",
        state: "abandoned",
        created: {
            year: 2003,
            month: 11,
            day: 13,
        },
        updated: {
            year: 2005,
            month: 6,
            day: 6,
        },
        origins: ["Harry Potter"],
        chapters: 11,
        tags: [{
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Nymphadora Tonks",
                type: TagType.Character,
            }],
    }, {
        id: "8671730",
        site: DlSite.FanFiction,
        rating: "mature",
        state: "abandoned",
        created: {
            year: 2012,
            month: 11,
            day: 4,
        },
        updated: {
            year: 2013,
            month: 8,
            day: 8,
        },
        origins: ["Harry Potter"],
        chapters: 9,
        tags: [{
                name: "Harry Potter/Nymphadora Tonks",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Nymphadora Tonks",
                type: TagType.Character,
            }, {
                name: "Andromeda Tonks",
                type: TagType.Character,
            }],
    }, {
        id: "8772113",
        site: DlSite.FanFiction,
        rating: "mature",
        state: "completed",
        created: {
            year: 2012,
            month: 12,
            day: 7,
        },
        updated: {
            year: 2015,
            month: 12,
            day: 15,
        },
        origins: ["Harry Potter"],
        chapters: 9,
        tags: [{
                name: "Harry Potter/Nymphadora Tonks",
                type: TagType.Pairing,
            }, {
                name: "Harry Potter",
                type: TagType.Character,
            }, {
                name: "Nymphadora Tonks",
                type: TagType.Character,
            }, {
                name: "Hermione Granger",
                type: TagType.Character,
            }],
    }];
for_story: for (let story of Stories) {
    npmlog_1.default.info(story.id, `Starting Story`);
    let json = {
        authors: new Array(),
        rating: story.rating,
        state: story.state,
        created: story.created,
        updated: story.updated,
        origins: story.origins,
        tags: story.tags,
        chapters: new Array(),
    };
    enum_switch: switch (story.site) {
        case DlSite.ArchiveOfOurOwn: {
            break enum_switch;
        }
        case DlSite.FanFiction: {
            for (let chapter = 1; chapter < story.chapters + 1; chapter++) {
                npmlog_1.default.info(story.id, `Starting Story Chapter: ${chapter}`);
                let res = sync_request_1.default("GET", `https://www.fanfiction.net/s/${story.id}/${chapter}/`, {
                    headers: {
                        "Accept": "text/html, application/xhtml+xml, application/xml; q=0.9, */*; q=0.8",
                        "Accept-Encoding": "gzip, deflate, br",
                        "Accept-Language": "en-US",
                        "Cache-Control": "max-age=0",
                        "Cookie": "cookies=yes",
                        "Host": "www.fanfiction.net",
                        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/64.0.3282.140 Safari/537.36 Edge/18.17763",
                    },
                });
                npmlog_1.default.info(`${story.id}/${chapter}`, `Returned Response`);
                if (res.statusCode != 200) {
                    npmlog_1.default.warn(`${story.id}/${chapter}`, `${res.statusCode}`);
                    npmlog_1.default.warn(`${story.id}/${chapter}`, res.getBody("utf8"));
                    break for_story;
                }
                else {
                    const $ = cheerio_1.default.load(res.getBody("utf8"));
                    if (chapter == 1) {
                        json.name = $("#profile_top > b.xcontrast_txt").text();
                        json.authors[0] = $("#profile_top > a.xcontrast_txt:not([title])").text();
                        json.summary = $("#profile_top > div.xcontrast_txt").text();
                    }
                    const chapter_text = $("#storytext").html();
                    if (chapter_text != null) {
                        json.chapters[chapter] = {
                            name: $("select#chap_select > option[selected]").first().text(),
                            place: chapter,
                            raw: Service.turndown(chapter_text),
                        };
                    }
                    else {
                        npmlog_1.default.error(`${story.id}/${chapter}`, "chapter_text is null");
                        fs_1.default.writeFileSync(`./import/err-story-${story.id}.json`, res.getBody("utf8"));
                        break for_story;
                    }
                }
                npmlog_1.default.info(`${story.id}/${chapter}`, `Finished Story Chapter`);
                sleep(random(10, 30) * 1000);
            }
            break enum_switch;
        }
        default:
            break enum_switch;
    }
    npmlog_1.default.info(story.id, `Finished Story`);
    sleep(random(10, 30) * 1000);
    fs_1.default.writeFileSync(`./import/story-${story.id}.json`, JSON.stringify(json, null, 4));
}
