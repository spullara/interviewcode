package interviewkotlin

import interview.Entity
import interview.Renderer
import java.util.*

class KotlinStringBuilderReplaceWithCodePoints : Renderer {

    override fun render(text: CharSequence, entities: Set<Entity>): CharSequence {
        val array = entities.toTypedArray()
        Arrays.sort<Entity>(array) { o1, o2 -> o2.start - o1.start }
        val sb = StringBuilder(text.length * 2).append(text)
        val s = text.toString()
        var pos = 0
        var codePointPosition = 0
        for (entity in array) {
            val start = s.offsetByCodePoints(pos, entity.start - codePointPosition)
            val end = s.offsetByCodePoints(pos, entity.end - codePointPosition)
            sb.replace(start, end, entity.html.toString())
            codePointPosition = entity.end
            pos = end
        }
        return sb
    }
}
