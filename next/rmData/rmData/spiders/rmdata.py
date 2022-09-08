import scrapy


class rmDataScraper(scrapy.Spider):
    name = "rmData"
    start_urls = [
        r"""http://data.people.com.cn/rmrb/s?qs=%7B%22cds%22%3A%5B%7B%22cdr%22%3A%22AND%22%2C%22cds%22%3A%5B%7B%22fld%22%3A%22title%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E7%A7%91%E6%8A%80%E5%A4%A7%E5%AD%A6%22%7D%2C%7B%22fld%22%3A%22subTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E7%A7%91%E6%8A%80%E5%A4%A7%E5%AD%A6%22%7D%2C%7B%22fld%22%3A%22introTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E7%A7%91%E6%8A%80%E5%A4%A7%E5%AD%A6%22%7D%5D%7D%5D%2C%22obs%22%3A%5B%7B%22fld%22%3A%22dataTime%22%2C%22drt%22%3A%22DESC%22%7D%5D%7D&tr=A&ss=1&pageNo=1&pageSize=500""",
        r"""http://data.people.com.cn/rmrb/s?qs=%7B%22cds%22%3A%5B%7B%22cdr%22%3A%22AND%22%2C%22cds%22%3A%5B%7B%22fld%22%3A%22title%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E5%B7%A5%E5%AD%A6%E9%99%A2%22%7D%2C%7B%22fld%22%3A%22subTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E5%B7%A5%E5%AD%A6%E9%99%A2%22%7D%2C%7B%22fld%22%3A%22introTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E5%B7%A5%E5%AD%A6%E9%99%A2%22%7D%5D%7D%5D%2C%22obs%22%3A%5B%7B%22fld%22%3A%22dataTime%22%2C%22drt%22%3A%22DESC%22%7D%5D%7D&tr=A&ss=1&pageNo=1&pageSize=500""",
        r"""http://data.people.com.cn/rmrb/s?qs=%7B%22cds%22%3A%5B%7B%22cdr%22%3A%22AND%22%2C%22cds%22%3A%5B%7B%22fld%22%3A%22title%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E7%90%86%E5%B7%A5%E5%A4%A7%E5%AD%A6%22%7D%2C%7B%22fld%22%3A%22subTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E7%90%86%E5%B7%A5%E5%A4%A7%E5%AD%A6%22%7D%2C%7B%22fld%22%3A%22introTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%8D%8E%E4%B8%AD%E7%90%86%E5%B7%A5%E5%A4%A7%E5%AD%A6%22%7D%5D%7D%5D%2C%22obs%22%3A%5B%7B%22fld%22%3A%22dataTime%22%2C%22drt%22%3A%22DESC%22%7D%5D%7D&tr=A&ss=1&pageNo=1&pageSize=500""",
        r"""http://data.people.com.cn/rmrb/s?qs=%7B%22cds%22%3A%5B%7B%22cdr%22%3A%22AND%22%2C%22cds%22%3A%5B%7B%22fld%22%3A%22title%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E6%AD%A6%E6%B1%89%E5%8C%BB%E5%AD%A6%E9%99%A2%22%7D%2C%7B%22fld%22%3A%22subTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E6%AD%A6%E6%B1%89%E5%8C%BB%E5%AD%A6%E9%99%A2%22%7D%2C%7B%22fld%22%3A%22introTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E6%AD%A6%E6%B1%89%E5%8C%BB%E5%AD%A6%E9%99%A2%22%7D%5D%7D%5D%2C%22obs%22%3A%5B%7B%22fld%22%3A%22dataTime%22%2C%22drt%22%3A%22DESC%22%7D%5D%7D&tr=A&ss=1&pageNo=1&pageSize=500""",
        r"""http://data.people.com.cn/rmrb/s?qs=%7B%22cds%22%3A%5B%7B%22cdr%22%3A%22AND%22%2C%22cds%22%3A%5B%7B%22fld%22%3A%22title%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%90%8C%E6%B5%8E%E5%8C%BB%E5%AD%A6%E9%99%A2%22%7D%2C%7B%22fld%22%3A%22subTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%90%8C%E6%B5%8E%E5%8C%BB%E5%AD%A6%E9%99%A2%22%7D%2C%7B%22fld%22%3A%22introTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%90%8C%E6%B5%8E%E5%8C%BB%E5%AD%A6%E9%99%A2%22%7D%5D%7D%5D%2C%22obs%22%3A%5B%7B%22fld%22%3A%22dataTime%22%2C%22drt%22%3A%22DESC%22%7D%5D%7D&tr=A&ss=1&pageNo=1&pageSize=500""",
        r"""http://data.people.com.cn/rmrb/s?qs=%7B%22cds%22%3A%5B%7B%22cdr%22%3A%22AND%22%2C%22cds%22%3A%5B%7B%22fld%22%3A%22title%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%90%8C%E6%B5%8E%E5%8C%BB%E7%A7%91%E5%A4%A7%E5%AD%A6%22%7D%2C%7B%22fld%22%3A%22subTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%90%8C%E6%B5%8E%E5%8C%BB%E7%A7%91%E5%A4%A7%E5%AD%A6%22%7D%2C%7B%22fld%22%3A%22introTitle%22%2C%22cdr%22%3A%22OR%22%2C%22hlt%22%3A%22true%22%2C%22vlr%22%3A%22OR%22%2C%22val%22%3A%22%E5%90%8C%E6%B5%8E%E5%8C%BB%E7%A7%91%E5%A4%A7%E5%AD%A6%22%7D%5D%7D%5D%2C%22obs%22%3A%5B%7B%22fld%22%3A%22dataTime%22%2C%22drt%22%3A%22DESC%22%7D%5D%7D&tr=A&ss=1&pageNo=1&pageSize=500""",
    ]

    keywords = ["华中工学院", "华中科技大学", "华中理工大学", "武汉医学院", "同济医科大学", "同济医学院"]

    def parse(self, response):
        for article in response.css(".sreach_li"):
            title = "".join(
                article.xpath(".//a[@class='open_detail_link']//text()").getall()
            )
            print(title)

            found = False
            for keyword in self.keywords:
                if keyword in title:
                    found = True
                    break

            if found:
                relative_url = article.css("a.open_detail_link::attr('href')").get()
                url = response.urljoin(relative_url)

                source = article.css("div.listinfo::text").get().strip()

                yield {"title": title, "url": url, "source": source}
