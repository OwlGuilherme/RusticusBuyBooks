import scrapy


class BretasSpider(scrapy.Spider):
    name = "bretas"
    allowed_domains = ["bretas.com.br"]
    start_urls = ["https://bretas.com.br/"]

    def parse(self, response):
        pass
